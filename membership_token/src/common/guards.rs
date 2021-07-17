use ic_cdk::caller;

use crate::common::utils::is_account_eq_principal;
use crate::get_state;

pub fn issue_guard() -> Result<(), String> {
    if is_account_eq_principal(get_state().controllers.issue_controller, caller()) {
        Ok(())
    } else {
        Err(String::from("The caller is not the issue controller"))
    }
}

pub fn revoke_guard() -> Result<(), String> {
    if is_account_eq_principal(get_state().controllers.revoke_controller, caller()) {
        Ok(())
    } else {
        Err(String::from("The caller is not the revoke controller"))
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
