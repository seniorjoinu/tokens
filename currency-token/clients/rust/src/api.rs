use ic_cdk::api::call::CallResult;
use ic_cdk::call;
use ic_cdk::export::candid::Principal;

use crate::types::{
    BurnRequest, DequeueRecurrentTaskRequest, DequeueRecurrentTaskResponse, GetBalanceOfRequest,
    GetBalanceOfResponse, GetControllersResponse, GetInfoResponse, GetRecurrentMintTasksResponse,
    GetRecurrentTransferTasksRequest, GetRecurrentTransferTasksResponse, GetTotalSupplyResponse,
    TransferRequest, UpdateControllersRequest, UpdateControllersResponse, UpdateInfoRequest,
    UpdateInfoResponse,
};

/// Client struct for easier interaction from other rust canisters
pub struct CurrencyTokenClient {
    pub canister_id: Principal,
}

impl CurrencyTokenClient {
    #[inline(always)]
    pub fn new(canister_id: Principal) -> Self {
        CurrencyTokenClient { canister_id }
    }

    #[inline(always)]
    pub async fn mint(&self, request: TransferRequest) -> CallResult<()> {
        call(self.canister_id, "mint", (request,)).await
    }

    #[inline(always)]
    pub async fn transfer(&self, request: TransferRequest) -> CallResult<()> {
        call(self.canister_id, "transfer", (request,)).await
    }

    #[inline(always)]
    pub async fn burn(&self, request: BurnRequest) -> CallResult<()> {
        call(self.canister_id, "burn", (request,)).await
    }

    #[inline(always)]
    pub async fn get_balance_of(
        &self,
        request: GetBalanceOfRequest,
    ) -> CallResult<(GetBalanceOfResponse,)> {
        call(self.canister_id, "get_balance_of", (request,)).await
    }

    #[inline(always)]
    pub async fn get_total_supply(&self) -> CallResult<(GetTotalSupplyResponse,)> {
        call(self.canister_id, "get_total_supply", ()).await
    }

    #[inline(always)]
    pub async fn get_info(&self) -> CallResult<(GetInfoResponse,)> {
        call(self.canister_id, "get_info", ()).await
    }

    #[inline(always)]
    pub async fn update_info(
        &self,
        request: UpdateInfoRequest,
    ) -> CallResult<(UpdateInfoResponse,)> {
        call(self.canister_id, "update_info", (request,)).await
    }

    #[inline(always)]
    pub async fn get_controllers(&self) -> CallResult<(GetControllersResponse,)> {
        call(self.canister_id, "get_controllers", ()).await
    }

    #[inline(always)]
    pub async fn update_info_controller(
        &self,
        request: UpdateControllersRequest,
    ) -> CallResult<(UpdateControllersResponse,)> {
        call(self.canister_id, "update_info_controller", (request,)).await
    }

    #[inline(always)]
    pub async fn update_mint_controller(
        &self,
        request: UpdateControllersRequest,
    ) -> CallResult<(UpdateControllersResponse,)> {
        call(self.canister_id, "update_mint_controller", (request,)).await
    }

    #[inline(always)]
    pub async fn dequeue_recurrent_transfer_tasks(
        &self,
        request: DequeueRecurrentTaskRequest,
    ) -> CallResult<(DequeueRecurrentTaskResponse,)> {
        call(
            self.canister_id,
            "dequeue_recurrent_transfer_tasks",
            (request,),
        )
        .await
    }

    #[inline(always)]
    pub async fn get_recurrent_transfer_tasks(
        &self,
        request: GetRecurrentTransferTasksRequest,
    ) -> CallResult<(GetRecurrentTransferTasksResponse,)> {
        call(self.canister_id, "get_recurrent_transfer_tasks", (request,)).await
    }

    #[inline(always)]
    pub async fn dequeue_recurrent_mint_tasks(
        &self,
        request: DequeueRecurrentTaskRequest,
    ) -> CallResult<(DequeueRecurrentTaskResponse,)> {
        call(self.canister_id, "dequeue_recurrent_mint_tasks", (request,)).await
    }

    #[inline(always)]
    pub async fn get_recurrent_mint_tasks(&self) -> CallResult<(GetRecurrentMintTasksResponse,)> {
        call(self.canister_id, "get_recurrent_mint_tasks", ()).await
    }
}
