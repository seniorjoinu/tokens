use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

use ic_cdk::export::candid::{decode_args, export_service};
use ic_cdk::{caller, id, trap};
use ic_cdk_macros::{init, query, update};
use ic_cron::implement_cron;
use ic_cron::types::ScheduledTask;
use ic_event_hub_macros::{
    implement_become_event_listener, implement_event_emitter, implement_get_event_listeners,
    implement_stop_being_event_listener,
};
use union_utils::{log, RemoteCallEndpoint};

use antifragile_currency_token_client::events::{
    ControllerType, ControllersUpdateEvent, InfoUpdateEvent, TokenMoveEvent,
};
use antifragile_currency_token_client::types::{
    BurnRequest, ControllerList, DequeueRecurrentTaskRequest, DequeueRecurrentTaskResponse,
    GetBalanceOfRequest, GetBalanceOfResponse, GetControllersResponse, GetInfoResponse,
    GetRecurrentMintTasksResponse, GetRecurrentTransferTasksRequest,
    GetRecurrentTransferTasksResponse, GetTotalSupplyResponse, InitRequest, RecurrentMintTaskExt,
    RecurrentTransferTaskExt, TransferRequest, UpdateControllersRequest, UpdateControllersResponse,
    UpdateInfoRequest, UpdateInfoResponse,
};

use crate::common::currency_token::CurrencyToken;
use crate::common::guards::{info_guard, mint_guard, self_guard};
use crate::common::types::{CronTaskKind, RecurrentMintTask, RecurrentTransferTask};

mod common;

// ----------------- MAIN LOGIC ------------------

#[init]
fn init(request: InitRequest) {
    log("currency_token.init()");

    let controllers = if let Some(default_controllers) = request.default_controllers {
        ControllerList {
            mint_controllers: default_controllers.clone(),
            info_controllers: default_controllers,
        }
    } else {
        ControllerList::single(Some(caller()))
    };

    let token = CurrencyToken {
        balances: HashMap::new(),
        total_supply: 0,
        info: request.info,
        controllers,
        recurrent_mint_tasks: HashSet::new(),
        recurrent_transfer_tasks: HashMap::new(),
    };

    unsafe {
        STATE = Some(token);
    }
}

#[update(guard = "mint_guard")]
fn mint(request: TransferRequest) {
    log("currency_token.mint()");

    let token = get_token();

    for (idx, entry) in request.entries.into_iter().enumerate() {
        match token.mint(entry.to, entry.qty) {
            Ok(_) => {
                if let Some(recurrence) = entry.recurrence {
                    let enqueue_result = cron_enqueue(
                        CronTaskKind::RecurrentMint as u8,
                        RecurrentMintTask {
                            to: entry.to,
                            qty: entry.qty,
                            event_payload: entry.event_payload.clone(),
                        },
                        recurrence,
                    );

                    match enqueue_result {
                        Ok(task_id) => {
                            token.register_recurrent_mint_task(task_id);
                        }
                        Err(_) => {
                            log("Candid serialization error met during recurrent mint enqueue");
                        }
                    };
                }

                emit(TokenMoveEvent {
                    from: None,
                    to: Some(entry.to),
                    qty: entry.qty,
                    event_payload: entry.event_payload,
                });
            }
            Err(e) => trap(format!("Error during minting entry #{} - {}", idx, e).as_str()),
        };
    }
}

#[update]
fn transfer(request: TransferRequest) {
    log("currency_token.transfer()");

    let token = get_token();
    let caller = caller();

    for (idx, entry) in request.entries.into_iter().enumerate() {
        match token.transfer(caller, entry.to, entry.qty) {
            Ok(_) => {
                if let Some(recurrence) = entry.recurrence {
                    let enqueue_result = cron_enqueue(
                        CronTaskKind::RecurrentTransfer as u8,
                        RecurrentTransferTask {
                            from: caller,
                            to: entry.to,
                            qty: entry.qty,
                            event_payload: entry.event_payload.clone(),
                        },
                        recurrence,
                    );

                    match enqueue_result {
                        Ok(task_id) => {
                            token.register_recurrent_transfer_task(caller, task_id);
                            log("Successfully registered transfer task");
                        }
                        Err(_) => {
                            log("Candid serialization error met during recurrent transfer enqueue");
                        }
                    };
                } else {
                    log("Recurrence is not provided");
                }

                emit(TokenMoveEvent {
                    from: Some(caller),
                    to: Some(entry.to),
                    qty: entry.qty,
                    event_payload: entry.event_payload,
                });
            }
            Err(e) => trap(format!("Error during transferring entry #{} - {}", idx, e).as_str()),
        };
    }
}

#[update]
fn burn(request: BurnRequest) {
    log("currency_token.burn()");

    let caller = caller();

    match get_token().burn(caller, request.qty) {
        Ok(_) => emit(TokenMoveEvent {
            from: Some(caller),
            to: None,
            qty: request.qty,
            event_payload: request.event_payload,
        }),
        Err(e) => trap(format!("Burning failed - {}", e).as_str()),
    }
}

#[query]
fn get_balance_of(request: GetBalanceOfRequest) -> GetBalanceOfResponse {
    log("currency_token.get_balance_of()");

    let balance = get_token().balance_of(&request.account_owner);

    GetBalanceOfResponse { balance }
}

#[query]
fn get_total_supply() -> GetTotalSupplyResponse {
    log("currency_token.get_total_supply()");

    let total_supply = get_token().total_supply;

    GetTotalSupplyResponse { total_supply }
}

#[query]
fn get_info() -> GetInfoResponse {
    log("currency_token.get_info()");

    let info = get_token().info.clone();

    GetInfoResponse { info }
}

#[update(guard = "info_guard")]
fn update_info(request: UpdateInfoRequest) -> UpdateInfoResponse {
    log("currency_token.update_info()");

    let old_info = get_token().update_info(request.new_info.clone());

    emit(InfoUpdateEvent {
        new_info: request.new_info,
    });

    UpdateInfoResponse { old_info }
}

// ------------- GRANULAR CONTROL --------------------

#[query]
fn get_controllers() -> GetControllersResponse {
    log("currency_token.get_controllers()");

    let controllers = get_token().controllers.clone();

    GetControllersResponse { controllers }
}

#[update(guard = "info_guard")]
fn update_info_controller(request: UpdateControllersRequest) -> UpdateControllersResponse {
    log("currency_token.update_info_controller()");

    let old_controller = get_token().update_info_controllers(request.new_controllers.clone());

    emit(ControllersUpdateEvent {
        kind: ControllerType::Info,
        new_controllers: request.new_controllers,
    });

    UpdateControllersResponse {
        old_controllers: old_controller,
    }
}

#[update(guard = "mint_guard")]
fn update_mint_controller(request: UpdateControllersRequest) -> UpdateControllersResponse {
    log("currency_token.update_mint_controller()");

    let old_controller = get_token().update_mint_controllers(request.new_controllers.clone());

    emit(ControllersUpdateEvent {
        kind: ControllerType::Mint,
        new_controllers: request.new_controllers,
    });

    UpdateControllersResponse {
        old_controllers: old_controller,
    }
}

// --------------- RECURRENCE ------------------

implement_cron!();

fn _cron_task_handler(task: ScheduledTask) {
    match task.get_kind().try_into() {
        Ok(CronTaskKind::RecurrentTransfer) => _recurrent_transfer(task.get_payload().unwrap()),
        Ok(CronTaskKind::RecurrentMint) => _recurrent_mint(task.get_payload().unwrap()),
        Err(_) => log("Invalid cron task handler"),
    }
}

fn _recurrent_transfer(task: RecurrentTransferTask) {
    log("currency_token._recurrent_transfer()");

    match get_token().transfer(task.from, task.to, task.qty) {
        Ok(_) => {
            emit(TokenMoveEvent {
                from: Some(task.from),
                to: Some(task.to),
                qty: task.qty,
                event_payload: task.event_payload,
            });
        }
        Err(e) => log(format!("Recurrent transferring failed with error: {}", e).as_str()),
    };
}

fn _recurrent_mint(task: RecurrentMintTask) {
    log("currency_token._recurrent_mint()");

    match get_token().mint(task.to, task.qty) {
        Ok(_) => {
            emit(TokenMoveEvent {
                from: None,
                to: Some(task.to),
                qty: task.qty,
                event_payload: task.event_payload,
            });
        }
        Err(e) => log(format!("Recurrent minting failed with error: {}", e).as_str()),
    };
}

#[update]
fn dequeue_recurrent_transfer_tasks(
    request: DequeueRecurrentTaskRequest,
) -> DequeueRecurrentTaskResponse {
    log("currency_token.dequeue_recurrent_transfer_tasks()");

    let caller = caller();
    let mut succeed = vec![];

    for task_id in request.task_ids {
        if get_token().unregister_recurrent_transfer_task(caller, task_id) {
            cron_dequeue(task_id);
            succeed.push(true);

            continue;
        }

        succeed.push(false);
    }

    DequeueRecurrentTaskResponse { succeed }
}

#[query]
fn get_recurrent_transfer_tasks(
    request: GetRecurrentTransferTasksRequest,
) -> GetRecurrentTransferTasksResponse {
    log("currency_token.get_recurrent_transfer_tasks()");

    let cron = get_cron_state();

    let tasks = get_token()
        .get_recurrent_transfer_tasks(request.owner)
        .into_iter()
        .map(|id| {
            let task = cron.scheduler.get_task_by_id(&id).unwrap();
            let task_payload = task.get_payload::<RecurrentTransferTask>().unwrap();

            RecurrentTransferTaskExt {
                task_id: task.id,
                from: task_payload.from,
                to: task_payload.to,
                qty: task_payload.qty,
                event_payload: task_payload.event_payload,
                scheduled_at: task.scheduled_at,
                rescheduled_at: task.rescheduled_at,
                scheduling_interval: task.scheduling_interval,
            }
        })
        .collect();

    GetRecurrentTransferTasksResponse { tasks }
}

#[update(guard = "mint_guard")]
fn dequeue_recurrent_mint_tasks(
    request: DequeueRecurrentTaskRequest,
) -> DequeueRecurrentTaskResponse {
    log("currency_token.dequeue_recurrent_mint_tasks()");

    let mut succeed = vec![];

    for task_id in request.task_ids {
        if get_token().unregister_recurrent_mint_task(task_id) {
            cron_dequeue(task_id);
            succeed.push(true);

            continue;
        }

        succeed.push(false);
    }

    DequeueRecurrentTaskResponse { succeed }
}

#[query]
fn get_recurrent_mint_tasks() -> GetRecurrentMintTasksResponse {
    log("currency_token.get_recurrent_mint_tasks()");

    let cron = get_cron_state();

    let tasks = get_token()
        .get_recurrent_mint_tasks()
        .into_iter()
        .map(|id| {
            let task = cron.scheduler.get_task_by_id(&id).unwrap();
            let task_payload = task.get_payload::<RecurrentMintTask>().unwrap();

            RecurrentMintTaskExt {
                task_id: task.id,
                to: task_payload.to,
                qty: task_payload.qty,
                event_payload: task_payload.event_payload,
                scheduled_at: task.scheduled_at,
                rescheduled_at: task.rescheduled_at,
                scheduling_interval: task.scheduling_interval,
            }
        })
        .collect();

    GetRecurrentMintTasksResponse { tasks }
}

// ------------------ EVENT HUB --------------------

implement_event_emitter!();
implement_become_event_listener!();
implement_stop_being_event_listener!();
implement_get_event_listeners!();

// ------------------ STATE ----------------------

export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

static mut STATE: Option<CurrencyToken> = None;

pub fn get_token() -> &'static mut CurrencyToken {
    unsafe { STATE.as_mut().unwrap() }
}
