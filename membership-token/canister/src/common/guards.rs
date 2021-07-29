use ic_cdk::caller;

use crate::get_state;

pub fn issue_guard() -> Result<(), String> {
    if get_state()
        .controllers
        .issue_controllers
        .contains(&caller())
    {
        Ok(())
    } else {
        Err(String::from("The caller is not the issue controller"))
    }
}

pub fn revoke_guard() -> Result<(), String> {
    if get_state()
        .controllers
        .revoke_controllers
        .contains(&caller())
    {
        Ok(())
    } else {
        Err(String::from("The caller is not the revoke controller"))
    }
}

pub fn event_listeners_guard() -> Result<(), String> {
    if get_state()
        .controllers
        .event_listeners_controllers
        .contains(&caller())
    {
        Ok(())
    } else {
        Err(String::from(
            "The caller is not the event listeners controller",
        ))
    }
}
