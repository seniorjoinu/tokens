use ic_cdk::caller;

use crate::get_state;

pub fn mint_guard() -> Result<(), String> {
    if get_state().controllers.mint_controllers.contains(&caller()) {
        Ok(())
    } else {
        Err(String::from("The caller is not the mint controller"))
    }
}

pub fn info_guard() -> Result<(), String> {
    if get_state().controllers.info_controllers.contains(&caller()) {
        Ok(())
    } else {
        Err(String::from("The caller is not the info controller"))
    }
}
