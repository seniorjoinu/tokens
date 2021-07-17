use ic_cdk::caller;

use crate::common::utils::is_account_eq_principal;
use crate::get_state;

pub fn mint_guard() -> Result<(), String> {
    if is_account_eq_principal(get_state().controllers.mint_controller, caller()) {
        Ok(())
    } else {
        Err(String::from("The caller is not the mint controller"))
    }
}

pub fn info_guard() -> Result<(), String> {
    if is_account_eq_principal(get_state().controllers.info_controller, caller()) {
        Ok(())
    } else {
        Err(String::from("The caller is not the info controller"))
    }
}

pub fn event_listeners_guard() -> Result<(), String> {
    if is_account_eq_principal(get_state().controllers.event_listeners_controller, caller()) {
        Ok(())
    } else {
        Err(String::from(
            "The caller is not the event listeners controller",
        ))
    }
}
