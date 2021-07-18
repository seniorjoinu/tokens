use ic_cdk::export::candid::Principal;
use ic_cdk::{caller, print};

use crate::common::types::Account;

pub fn log(msg: &str) {
    print(format!("[caller: {}]: {}", caller(), msg))
}

pub fn is_account_eq_principal(acc: Account, prin: Principal) -> bool {
    if let Some(acc_prin) = acc {
        acc_prin == prin
    } else {
        false
    }
}
