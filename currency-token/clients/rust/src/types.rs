use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_cron::types::{SchedulingInterval, TaskId};

pub type Controllers = Vec<Principal>;
pub type Payload = Option<Vec<u8>>;

#[derive(Clone, CandidType, Deserialize)]
pub struct ControllerList {
    pub mint_controllers: Controllers,
    pub info_controllers: Controllers,
}

impl ControllerList {
    pub fn single(controller_opt: Option<Principal>) -> ControllerList {
        let controllers = if let Some(controller) = controller_opt {
            vec![controller]
        } else {
            Vec::new()
        };

        ControllerList {
            mint_controllers: controllers.clone(),
            info_controllers: controllers,
        }
    }
}

#[derive(CandidType, Deserialize)]
pub struct TransferEntry {
    pub to: Principal,
    pub qty: u64,
    pub event_payload: Payload,
    pub recurrence: Option<SchedulingInterval>,
}

#[derive(CandidType, Deserialize)]
pub struct DequeueRecurrentTaskRequest {
    pub task_ids: Vec<TaskId>,
}

#[derive(CandidType, Deserialize)]
pub struct DequeueRecurrentTaskResponse {
    pub succeed: Vec<bool>,
}

#[derive(CandidType, Deserialize)]
pub struct GetRecurrentTransferTasksRequest {
    pub owner: Principal,
}

#[derive(Clone, CandidType, Deserialize)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(CandidType, Deserialize)]
pub struct InitRequest {
    pub info: TokenInfo,
    pub default_controllers: Option<Controllers>,
}

#[derive(CandidType, Deserialize)]
pub struct GetBalanceOfRequest {
    pub account_owner: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct GetBalanceOfResponse {
    pub balance: u64,
}

#[derive(CandidType, Deserialize)]
pub struct GetTotalSupplyResponse {
    pub total_supply: u64,
}

#[derive(CandidType, Deserialize)]
pub struct GetInfoResponse {
    pub info: TokenInfo,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateInfoRequest {
    pub new_info: TokenInfo,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateInfoResponse {
    pub old_info: TokenInfo,
}

#[derive(CandidType, Deserialize)]
pub struct GetControllersResponse {
    pub controllers: ControllerList,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateControllersRequest {
    pub new_controllers: Controllers,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateControllersResponse {
    pub old_controllers: Controllers,
}

#[derive(CandidType, Deserialize)]
pub struct TransferRequest {
    pub entries: Vec<TransferEntry>,
}

#[derive(CandidType, Deserialize)]
pub struct BurnRequest {
    pub qty: u64,
    pub event_payload: Payload,
}

#[derive(CandidType, Deserialize)]
pub struct RecurrentTransferTaskExt {
    pub task_id: TaskId,
    pub from: Principal,
    pub to: Principal,
    pub qty: u64,
    pub event_payload: Payload,
    pub scheduled_at: u64,
    pub rescheduled_at: Option<u64>,
    pub scheduling_interval: SchedulingInterval,
}

#[derive(CandidType, Deserialize)]
pub struct RecurrentMintTaskExt {
    pub task_id: TaskId,
    pub to: Principal,
    pub qty: u64,
    pub event_payload: Payload,
    pub scheduled_at: u64,
    pub rescheduled_at: Option<u64>,
    pub scheduling_interval: SchedulingInterval,
}

#[derive(CandidType, Deserialize)]
pub struct GetRecurrentTransferTasksResponse {
    pub tasks: Vec<RecurrentTransferTaskExt>,
}

#[derive(CandidType, Deserialize)]
pub struct GetRecurrentMintTasksResponse {
    pub tasks: Vec<RecurrentMintTaskExt>,
}
