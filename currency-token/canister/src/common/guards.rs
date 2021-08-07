use ic_cdk::{caller, id};

use crate::get_token;

#[inline(always)]
pub fn mint_guard() -> Result<(), String> {
    if get_token().controllers.mint_controllers.contains(&caller()) {
        Ok(())
    } else {
        Err(String::from("The caller is not the mint controller"))
    }
}

#[inline(always)]
pub fn info_guard() -> Result<(), String> {
    if get_token().controllers.info_controllers.contains(&caller()) {
        Ok(())
    } else {
        Err(String::from("The caller is not the info controller"))
    }
}

#[inline(always)]
pub fn self_guard() -> Result<(), String> {
    if caller() == id() {
        Ok(())
    } else {
        Err(String::from("The caller is not the Currency Token itself"))
    }
}
