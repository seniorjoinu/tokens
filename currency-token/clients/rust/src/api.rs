use ic_cdk::api::call::CallResult;
use ic_cdk::call;
use ic_cdk::export::candid::Principal;

use crate::types::{
    BurnRequest, BurnResponse, GetBalanceOfRequest, GetBalanceOfResponse, GetControllersResponse,
    GetInfoResponse, GetTotalSupplyResponse, TransferRequest, TransferResponse,
    UpdateControllersRequest, UpdateControllersResponse, UpdateInfoRequest, UpdateInfoResponse,
};

pub struct CurrencyTokenClient {
    pub canister_id: Principal,
}

impl CurrencyTokenClient {
    pub fn new(canister_id: Principal) -> Self {
        CurrencyTokenClient { canister_id }
    }

    pub async fn mint(&self, request: TransferRequest) -> CallResult<(TransferResponse,)> {
        call(self.canister_id, "mint", (request,)).await
    }

    pub async fn transfer(&self, request: TransferRequest) -> CallResult<(TransferResponse,)> {
        call(self.canister_id, "transfer", (request,)).await
    }

    pub async fn burn(&self, request: BurnRequest) -> CallResult<(BurnResponse,)> {
        call(self.canister_id, "burn", (request,)).await
    }

    pub async fn get_balance_of(
        &self,
        request: GetBalanceOfRequest,
    ) -> CallResult<(GetBalanceOfResponse,)> {
        call(self.canister_id, "get_balance_of", (request,)).await
    }

    pub async fn get_total_supply(&self) -> CallResult<(GetTotalSupplyResponse,)> {
        call(self.canister_id, "get_total_supply", ()).await
    }

    pub async fn get_info(&self) -> CallResult<(GetInfoResponse,)> {
        call(self.canister_id, "get_info", ()).await
    }

    pub async fn update_info(
        &self,
        request: UpdateInfoRequest,
    ) -> CallResult<(UpdateInfoResponse,)> {
        call(self.canister_id, "update_info", (request,)).await
    }

    pub async fn get_controllers(&self) -> CallResult<(GetControllersResponse,)> {
        call(self.canister_id, "get_controllers", ()).await
    }

    pub async fn update_info_controller(
        &self,
        request: UpdateControllersRequest,
    ) -> CallResult<(UpdateControllersResponse,)> {
        call(self.canister_id, "update_info_controller", (request,)).await
    }

    pub async fn update_mint_controller(
        &self,
        request: UpdateControllersRequest,
    ) -> CallResult<(UpdateControllersResponse,)> {
        call(self.canister_id, "update_mint_controller", (request,)).await
    }
}
