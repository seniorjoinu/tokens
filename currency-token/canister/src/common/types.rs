use std::fmt::{Display, Formatter};

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cron::u8_enum;

use currency_token_client::types::Payload;

pub enum Error {
    InsufficientBalance,
    ZeroQuantity,
    AccessDenied,
    ForbiddenOperation,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Error::InsufficientBalance => "InsufficientBalance",
            Error::ZeroQuantity => "ZeroQuantity",
            Error::AccessDenied => "AccessDenied",
            Error::ForbiddenOperation => "ForbiddenOperation",
        };

        f.write_str(str)
    }
}

u8_enum! {
    pub enum CronTaskKind {
        RecurrentTransfer,
        RecurrentMint,
    }
}

#[derive(CandidType, Deserialize)]
pub struct RecurrentTransferTask {
    pub from: Principal,
    pub to: Principal,
    pub qty: u64,
    pub event_payload: Payload,
}

#[derive(CandidType, Deserialize)]
pub struct RecurrentMintTask {
    pub to: Principal,
    pub qty: u64,
    pub event_payload: Payload,
}
