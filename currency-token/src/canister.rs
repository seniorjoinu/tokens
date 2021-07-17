use std::collections::HashMap;

use ic_cdk::{caller, trap};
use ic_cdk::export::candid::{export_service, Principal};
use ic_cdk_macros::{init, query, update};
use ic_event_hub::{implement_add_event_listeners, implement_become_event_listener, implement_event_emitter, implement_get_event_listeners, implement_remove_event_listeners};

use crate::common::guards::{event_listeners_guard, info_guard, mint_guard};
use crate::common::utils::{Account, Controllers, CurrencyToken, CurrencyTokenInfo, CurrencyTokenInitRequest, CurrencyTokenTransferEntry, Error, log};

mod common;

#[init]
fn init(request: CurrencyTokenInitRequest) {
    log("currency_token.init()");

    let controllers = Controllers::single(Account::Some(caller()));

    let mut token = CurrencyToken {
        balances: HashMap::new(),
        total_supply: 0,
        info: request.info,
        controllers,
    };

    unsafe {
        STATE = Some(token);
    }
}

#[query]
fn get_balance_of(account_owner: Principal) -> u64 {
    log("currency_token.get_balance_of()");

    get_state().balance_of(&account_owner)
}

#[query]
fn get_total_supply() -> u64 {
    log("currency_token.get_total_supply()");

    get_state().total_supply
}

#[query]
fn get_info() -> CurrencyTokenInfo {
    log("currency_token.info()");

    get_state().info.clone()
}

#[update(guard = "info_guard")]
fn update_info(new_info: CurrencyTokenInfo) -> CurrencyTokenInfo {
    log("currency_token.update_info()");

    get_state().update_info(new_info)
}

#[query]
fn controllers() -> Controllers {
    log("currency_token.controllers()");

    get_state().controllers.clone()
}

#[update(guard = "info_guard")]
fn update_info_controller(new_controller: Account) -> Account {
    log("currency_token.update_info_controller()");

    get_state().update_info_controller(new_controller)
}

#[update(guard = "mint_guard")]
fn update_mint_controller(new_controller: Account) -> Account {
    log("fungible_token.update_mint_controller()");

    get_state().update_mint_controller(new_controller)
}

#[update(guard = "event_listeners_guard")]
fn update_event_listeners_guard_controller(new_controller: Account) -> Account {
    log("currency_token.update_on_move_controller()");

    get_state().update_event_listeners_controller(new_controller)
}

#[update(guard = "mint_guard")]
async fn mint(entries: Vec<CurrencyTokenTransferEntry>) -> Vec<Result<(), Error>> {
    log("currency_token.mint()");

    let state = get_state();

    entries
        .into_iter()
        .map(|entry| {
            state.mint(entry.to, entry.qty, entry.payload).map(|(ev1, ev2)| {
                emit(ev1);
                emit(ev2);
            })
        })
        .collect()
}

#[update]
async fn send(entries: Vec<CurrencyTokenTransferEntry>) -> Vec<Result<(), Error>> {
    log("currency_token.send()");

    let state = get_state();

    entries
        .into_iter()
        .map(|entry| {
            state.transfer(caller(), entry.to, entry.qty, entry.payload).map(|(ev1, ev2, ev3)| {
                emit(ev1);
                emit(ev2);
                emit(ev3);
            })
        })
        .collect()
}

#[update]
async fn burn(quantity: u64, payload: Option<Vec<u8>>) -> Result<(), Error> {
    log("currency_token.burn()");

    let state = get_state();

    state.burn(caller(), quantity, payload).map(|(ev1, ev2)| {
        emit(ev1);
        emit(ev2);
    })
}

// ------------------ EVENT HUB --------------------

implement_event_emitter!();
implement_add_event_listeners!(guard = "event_listeners_guard");
implement_remove_event_listeners!(guard = "event_listeners_guard");
implement_become_event_listener!();
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