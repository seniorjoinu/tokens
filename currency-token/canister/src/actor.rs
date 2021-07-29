use std::collections::HashMap;

use ic_cdk::caller;
use ic_cdk::export::candid::export_service;
use ic_cdk_macros::{init, query, update};
use ic_event_hub_macros::{
    implement_become_event_listener, implement_event_emitter, implement_get_event_listeners,
    implement_stop_being_event_listener,
};
use union_utils::log;

use antifragile_currency_token_client::events::{
    ControllerType, ControllersUpdateEvent, InfoUpdateEvent,
};
use antifragile_currency_token_client::types::{
    BurnRequest, BurnResponse, ControllerList, CurrencyTokenInitRequest, GetBalanceOfRequest,
    GetBalanceOfResponse, GetControllersResponse, GetInfoResponse, GetTotalSupplyResponse,
    TransferRequest, TransferResponse, UpdateControllersRequest, UpdateControllersResponse,
    UpdateInfoRequest, UpdateInfoResponse,
};

use crate::common::currency_token::CurrencyToken;
use crate::common::guards::{info_guard, mint_guard};

mod common;

// ----------------- MAIN LOGIC ------------------

#[init]
fn init(request: CurrencyTokenInitRequest) {
    log("currency_token.init()");

    let controllers = ControllerList::single(Some(caller()));

    let token = CurrencyToken {
        balances: HashMap::new(),
        total_supply: 0,
        info: request.info,
        controllers,
    };

    unsafe {
        STATE = Some(token);
    }
}

#[update(guard = "mint_guard")]
fn mint(request: TransferRequest) -> TransferResponse {
    log("currency_token.mint()");

    let state = get_state();

    let results: Vec<_> = request
        .entries
        .into_iter()
        .map(|entry| {
            state
                .mint(entry.to, entry.qty, entry.payload)
                .map(|(ev1, ev2, ev3)| {
                    emit(ev1);
                    emit(ev2);
                    emit(ev3);
                })
        })
        .collect();

    TransferResponse { results }
}

#[update]
fn transfer(request: TransferRequest) -> TransferResponse {
    log("currency_token.transfer()");

    let state = get_state();

    let results: Vec<_> = request
        .entries
        .into_iter()
        .map(|entry| {
            state
                .transfer(caller(), entry.to, entry.qty, entry.payload)
                .map(|(ev1, ev2, ev3)| {
                    emit(ev1);
                    emit(ev2);
                    emit(ev3);
                })
        })
        .collect();

    TransferResponse { results }
}

#[update]
fn burn(request: BurnRequest) -> BurnResponse {
    log("currency_token.burn()");

    get_state()
        .burn(caller(), request.quantity, request.payload)
        .map(|(ev1, ev2, ev3)| {
            emit(ev1);
            emit(ev2);
            emit(ev3);
        })
}

#[query]
fn get_balance_of(request: GetBalanceOfRequest) -> GetBalanceOfResponse {
    log("currency_token.get_balance_of()");

    let balance = get_state().balance_of(&request.account_owner);

    GetBalanceOfResponse { balance }
}

#[query]
fn get_total_supply() -> GetTotalSupplyResponse {
    log("currency_token.get_total_supply()");

    let total_supply = get_state().total_supply;

    GetTotalSupplyResponse { total_supply }
}

#[query]
fn get_info() -> GetInfoResponse {
    log("currency_token.get_info()");

    let info = get_state().info.clone();

    GetInfoResponse { info }
}

#[update(guard = "info_guard")]
fn update_info(request: UpdateInfoRequest) -> UpdateInfoResponse {
    log("currency_token.update_info()");

    let old_info = get_state().update_info(request.new_info.clone());

    emit(InfoUpdateEvent {
        new_info: request.new_info,
    });

    UpdateInfoResponse { old_info }
}

// ------------- GRANULAR CONTROL --------------------

#[query]
fn get_controllers() -> GetControllersResponse {
    log("currency_token.get_controllers()");

    let controllers = get_state().controllers.clone();

    GetControllersResponse { controllers }
}

#[update(guard = "info_guard")]
fn update_info_controller(request: UpdateControllersRequest) -> UpdateControllersResponse {
    log("currency_token.update_info_controller()");

    let old_controller = get_state().update_info_controllers(request.new_controllers.clone());

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

    let old_controller = get_state().update_mint_controllers(request.new_controllers.clone());

    emit(ControllersUpdateEvent {
        kind: ControllerType::Mint,
        new_controllers: request.new_controllers,
    });

    UpdateControllersResponse {
        old_controllers: old_controller,
    }
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

pub fn get_state() -> &'static mut CurrencyToken {
    unsafe { STATE.as_mut().unwrap() }
}
