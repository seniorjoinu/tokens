use std::collections::HashMap;

use ic_cdk::{caller, print};
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_event_hub_macros::Event;

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

pub type Account = Option<Principal>;

#[derive(Clone, CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct Controllers {
    pub mint_controller: Account,
    pub info_controller: Account,
    pub event_listeners_controller: Account,
}

impl Controllers {
    pub fn single(controller: Account) -> Controllers {
        Controllers {
            mint_controller: controller,
            info_controller: controller,
            event_listeners_controller: controller,
        }
    }
}

#[derive(Clone, CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct CurrencyTokenTransferEntry {
    pub to: Principal,
    pub qty: u64,
    pub payload: Option<Vec<u8>>,
}

#[derive(Clone, CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct CurrencyTokenInitRequest {
    pub info: CurrencyTokenInfo,
}

#[derive(Clone, CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct CurrencyTokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Clone, CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub enum Error {
    InsufficientBalance,
    ZeroQuantity,
    AccessDenied,
    ForbiddenOperation,
}

#[derive(Clone, CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct CurrencyToken {
    pub balances: HashMap<Principal, u64>,
    pub total_supply: u64,
    pub info: CurrencyTokenInfo,
    pub controllers: Controllers,
}

pub type Payload = Option<Vec<u8>>;

#[derive(Event, CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct TokenMoveEvent {
    #[topic]
    from: Account,
    #[topic]
    to: Account,
    qty: u64,
    payload: Payload,
}

#[derive(Event, CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct VotingPowerUpdateEvent {
    #[topic]
    pub voter: Principal,
    pub new_voting_power: u64,
}

impl CurrencyToken {
    pub fn mint(
        &mut self,
        to: Principal,
        qty: u64,
        payload: Payload,
    ) -> Result<(TokenMoveEvent, VotingPowerUpdateEvent), Error> {
        if qty == 0 {
            return Err(Error::ZeroQuantity);
        }

        let prev_balance = self.balance_of(&to);
        let new_balance = prev_balance + qty;

        self.total_supply += qty;
        self.balances.insert(to, new_balance);

        Ok((
            TokenMoveEvent {
                from: Account::None,
                to: Account::Some(to),
                qty,
                payload,
            },
            VotingPowerUpdateEvent {
                voter: to,
                new_voting_power: new_balance,
            }
        ))
    }

    pub fn transfer(
        &mut self,
        from: Principal,
        to: Principal,
        qty: u64,
        payload: Payload,
    ) -> Result<(TokenMoveEvent, VotingPowerUpdateEvent, VotingPowerUpdateEvent), Error> {
        if qty == 0 {
            return Err(Error::ZeroQuantity);
        }

        let prev_from_balance = self.balance_of(&from);
        let prev_to_balance = self.balance_of(&to);

        if prev_from_balance < qty {
            return Err(Error::InsufficientBalance);
        }

        let new_from_balance = prev_from_balance - qty;
        let new_to_balance = prev_to_balance + qty;

        if new_from_balance == 0 {
            self.balances.remove(&from);
        } else {
            self.balances.insert(from, new_from_balance);
        }

        self.balances.insert(to, new_to_balance);

        Ok((
            TokenMoveEvent {
                from: Account::Some(from),
                to: Account::Some(to),
                qty,
                payload,
            },
            VotingPowerUpdateEvent {
                voter: from,
                new_voting_power: new_from_balance,
            },
            VotingPowerUpdateEvent {
                voter: to,
                new_voting_power: new_to_balance,
            }
        ))
    }

    pub fn burn(
        &mut self,
        from: Principal,
        qty: u64,
        payload: Payload,
    ) -> Result<(TokenMoveEvent, VotingPowerUpdateEvent), Error> {
        if qty == 0 {
            return Err(Error::ZeroQuantity);
        }

        let prev_balance = self.balance_of(&from);

        if prev_balance < qty {
            return Err(Error::InsufficientBalance);
        }

        let new_balance = prev_balance - qty;

        if new_balance == 0 {
            self.balances.remove(&from);
        } else {
            self.balances.insert(from, new_balance);
        }

        self.total_supply -= qty;

        Ok((
            TokenMoveEvent {
                from: Account::Some(from),
                to: Account::None,
                qty,
                payload,
            },
            VotingPowerUpdateEvent {
                voter: from,
                new_voting_power: new_balance,
            }
        ))
    }

    pub fn update_info(
        &mut self,
        new_info: CurrencyTokenInfo,
    ) -> CurrencyTokenInfo {
        let old_info = self.info.clone();
        self.info = new_info;

        old_info
    }

    pub fn update_mint_controller(
        &mut self,
        new_mint_controller: Account,
    ) -> Account {
        let old_controller = self.controllers.mint_controller;
        self.controllers.mint_controller = new_mint_controller;

        old_controller
    }

    pub fn update_event_listeners_controller(
        &mut self,
        new_event_listeners_controller: Account,
    ) -> Account {
        let old_controller = self.controllers.event_listeners_controller;
        self.controllers.event_listeners_controller = new_event_listeners_controller;

        old_controller
    }

    pub fn update_info_controller(
        &mut self,
        new_info_controller: Account,
    ) -> Account {
        let old_controller = self.controllers.info_controller;
        self.controllers.info_controller = new_info_controller;

        old_controller
    }

    pub fn balance_of(&self, account_owner: &Principal) -> u64 {
        match self.balances.get(&account_owner) {
            None => 0,
            Some(b) => *b,
        }
    }
}

#[cfg(test)]
mod fungible_token {
    use std::collections::HashMap;

    use union_utils::on_move::{OnMoveEventFilter, OnMoveEventListener, OnMoveEventListenersInfo};
    use union_utils::test_utils::{magic_blob, random_endpoint, random_principal};
    use union_utils::types::Account;

    use crate::utils::{Controllers, CurrencyToken, CurrencyTokenInfo};

    fn create_fungible_token() -> (CurrencyToken, Account) {
        let controller = Some(random_principal());
        let token = CurrencyToken {
            balances: HashMap::new(),
            total_supply: 0,
            info: CurrencyTokenInfo {
                name: String::from("test"),
                symbol: String::from("TST"),
                decimals: 8,
            },
            on_move_listeners: OnMoveEventListenersInfo::default(),
            controllers: Controllers::single(controller),
        };

        (token, controller)
    }

    #[test]
    fn creation_works_fine() {
        let (token, controller) = create_fungible_token();

        assert!(token.balances.is_empty());
        assert_eq!(token.total_supply, 0);
        assert_eq!(token.on_move_listeners.id_counter, 0);
        assert!(token.on_move_listeners.enumeration.is_empty());
        assert!(token.on_move_listeners.index.is_empty());
        assert_eq!(token.controllers.on_move_controller, controller);
        assert_eq!(token.controllers.info_controller, controller);
        assert_eq!(token.controllers.mint_controller, controller);
        assert_eq!(token.info.name, String::from("test"));
        assert_eq!(token.info.symbol, String::from("TST"));
        assert_eq!(token.info.decimals, 8);
    }

    #[test]
    fn minting_works_right() {
        let (mut token, controller) = create_fungible_token();
        let user_1 = random_principal();

        let ev_n_listeners_1 = token
            .mint(user_1, 100, controller.unwrap(), None)
            .expect("mint 1 should work");

        assert_eq!(token.total_supply, 100);
        assert_eq!(token.balances.len(), 1);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 100);

        assert!(ev_n_listeners_1.listeners.is_empty());
        assert_eq!(ev_n_listeners_1.event.from, None);
        assert_eq!(ev_n_listeners_1.event.to, Some(user_1));
        assert_eq!(ev_n_listeners_1.event.qty, 100);
        assert_eq!(ev_n_listeners_1.event.payload, None);

        let ev_n_listeners_2 = token
            .mint(
                controller.unwrap(),
                200,
                controller.unwrap(),
                Some(magic_blob()),
            )
            .expect("mint 2 should work");

        assert_eq!(token.total_supply, 300);
        assert_eq!(token.balances.len(), 2);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 100);
        assert_eq!(
            token.balances.get(&controller.unwrap()).unwrap().clone(),
            200
        );

        assert!(ev_n_listeners_2.listeners.is_empty());
        assert_eq!(ev_n_listeners_2.event.from, None);
        assert_eq!(ev_n_listeners_2.event.qty, 200);
        assert_eq!(ev_n_listeners_2.event.to, controller);
        assert_eq!(ev_n_listeners_2.event.payload, Some(magic_blob()));
    }

    #[test]
    fn burning_works_fine() {
        let (mut token, controller) = create_fungible_token();
        let user_1 = random_principal();

        token
            .mint(user_1, 100, controller.unwrap(), None)
            .expect("mint 1 should work");

        let ev_n_listeners_1 = token.burn(user_1, 90, None).expect("burn 1 should work");

        assert_eq!(token.balances.len(), 1);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 10);
        assert_eq!(token.total_supply, 10);

        assert!(ev_n_listeners_1.listeners.is_empty());
        assert_eq!(ev_n_listeners_1.event.from, Some(user_1));
        assert_eq!(ev_n_listeners_1.event.to, None);
        assert_eq!(ev_n_listeners_1.event.qty, 90);
        assert_eq!(ev_n_listeners_1.event.payload, None);

        token
            .burn(user_1, 20, None)
            .expect_err("overburn shouldn't work");

        let ev_n_listeners_2 = token.burn(user_1, 10, None).expect("burn 2 should work");

        assert!(token.balances.is_empty());
        assert!(token.balances.get(&user_1).is_none());
        assert_eq!(token.total_supply, 0);

        assert!(ev_n_listeners_2.listeners.is_empty());
        assert_eq!(ev_n_listeners_2.event.from, Some(user_1));
        assert_eq!(ev_n_listeners_2.event.to, None);
        assert_eq!(ev_n_listeners_2.event.qty, 10);
        assert_eq!(ev_n_listeners_2.event.payload, None);

        token
            .burn(user_1, 20, None)
            .expect_err("overburn shouldn't work");
    }

    #[test]
    fn transfer_works_fine() {
        let (mut token, controller) = create_fungible_token();
        let user_1 = random_principal();
        let user_2 = random_principal();

        token
            .mint(user_1, 1000, controller.unwrap(), None)
            .expect("mint 1 should work");

        let ev_n_listeners_1 = token
            .transfer(user_1, user_2, 100, None)
            .expect("transfer 1 should work");

        assert_eq!(token.balances.len(), 2);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 900);
        assert_eq!(token.balances.get(&user_2).unwrap().clone(), 100);
        assert_eq!(token.total_supply, 1000);

        assert!(ev_n_listeners_1.listeners.is_empty());
        assert_eq!(ev_n_listeners_1.event.from, Some(user_1));
        assert_eq!(ev_n_listeners_1.event.to, Some(user_2));
        assert_eq!(ev_n_listeners_1.event.qty, 100);
        assert_eq!(ev_n_listeners_1.event.payload, None);

        token
            .transfer(user_1, user_2, 1000, None)
            .expect_err("overtrasnfer 1 should fail");

        token
            .transfer(controller.unwrap(), user_2, 100, None)
            .expect_err("overtransfer 2 should fail");

        let ev_n_listeners_2 = token
            .transfer(user_2, user_1, 100, None)
            .expect("transfer 2 should work");

        assert_eq!(token.balances.len(), 1);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 1000);
        assert!(token.balances.get(&user_2).is_none());
        assert_eq!(token.total_supply, 1000);

        assert!(ev_n_listeners_2.listeners.is_empty());
        assert_eq!(ev_n_listeners_2.event.from, Some(user_2));
        assert_eq!(ev_n_listeners_2.event.to, Some(user_1));
        assert_eq!(ev_n_listeners_2.event.qty, 100);
        assert_eq!(ev_n_listeners_2.event.payload, None);

        token
            .transfer(user_2, user_1, 1, None)
            .expect_err("overtransfer 3 should fail");

        token
            .transfer(user_2, user_1, 0, None)
            .expect_err("transfer of zero tokens should fail");
    }

    #[test]
    fn info_update_works_well() {
        let (mut token, controller) = create_fungible_token();

        let new_info_1 = CurrencyTokenInfo {
            name: String::from("name 1"),
            symbol: String::from("NME1"),
            decimals: 9,
        };
        token
            .update_info(new_info_1, controller.unwrap())
            .expect("should change info 1");

        assert_eq!(token.info.name, String::from("name 1"));
        assert_eq!(token.info.symbol, String::from("NME1"));
        assert_eq!(token.info.decimals, 9);

        let new_info_2 = CurrencyTokenInfo {
            name: String::from("name 2"),
            symbol: String::from("NME2"),
            decimals: 2,
        };
        token
            .update_info(new_info_2, controller.unwrap())
            .expect("should change info 2");

        assert_eq!(token.info.name, String::from("name 2"));
        assert_eq!(token.info.symbol, String::from("NME2"));
        assert_eq!(token.info.decimals, 2);
    }

    #[test]
    fn access_control_works_fine() {
        let (mut token, controller) = create_fungible_token();
        let user_1 = random_principal();
        let user_2 = random_principal();

        assert_eq!(token.controllers.mint_controller, controller);
        assert_eq!(token.controllers.info_controller, controller);
        assert_eq!(token.controllers.on_move_controller, controller);

        // --- ISSUE ---
        token
            .mint(controller.unwrap(), 10, controller.unwrap(), None)
            .expect("should be possible to mint to the controller");
        token
            .mint(user_1, 10, user_1, None)
            .expect_err("should be impossible to mint by not the controller");

        token
            .update_mint_controller(Some(user_1), user_1)
            .expect_err("should be impossible to update mint controller by not the controller");
        token
            .update_mint_controller(Some(user_1), controller.unwrap())
            .expect("should be possible to update mint controller by the controller");

        token
            .mint(user_1, 10, user_1, None)
            .expect("should be possible to mint by user_1");
        token
            .mint(user_2, 10, controller.unwrap(), None)
            .expect_err("should be impossible to mint by the old controller");

        token
            .update_mint_controller(None, controller.unwrap())
            .expect_err("should be impossible to update mint controller by the old controller");
        token
            .update_mint_controller(None, user_1)
            .expect("should be possible to update mint controller by user_1");

        token
            .mint(user_2, 10, user_1, None)
            .expect_err("should be impossible to mint by anyone now");
        token
            .mint(user_2, 10, controller.unwrap(), None)
            .expect_err("should be impossible to mint by anyone now");
        token
            .mint(user_2, 10, user_2, None)
            .expect_err("should be impossible to mint by anyone now");

        // --- INFO ---
        let new_info_1 = CurrencyTokenInfo {
            name: String::from("new name 1"),
            symbol: String::from("abc"),
            decimals: 8,
        };
        token
            .update_info(new_info_1.clone(), user_1)
            .expect_err("should be impossible to change info by not the controller");
        token
            .update_info(new_info_1, controller.unwrap())
            .expect("should be possible to change info by the controller");

        token
            .update_info_controller(Some(user_1), user_1)
            .expect_err("should be impossible to update info controller by not the controller");
        token
            .update_info_controller(Some(user_1), controller.unwrap())
            .expect("should be possible to update info controller by the controller");

        let new_info_2 = CurrencyTokenInfo {
            name: String::from("new name 2"),
            symbol: String::from("abc"),
            decimals: 8,
        };
        token
            .update_info(new_info_2.clone(), controller.unwrap())
            .expect_err("should be impossible to update info by the old controller");
        token
            .update_info(new_info_2, user_1)
            .expect("should be possible to update info by user_1");

        token
            .update_info_controller(None, controller.unwrap())
            .expect_err("should be impossible to update info controller by the old controller");
        token
            .update_info_controller(None, user_1)
            .expect("should be possible to update info controller by user_1");

        let new_info_3 = CurrencyTokenInfo {
            name: String::from("new name 3"),
            symbol: String::from("abc"),
            decimals: 8,
        };
        token
            .update_info(new_info_3.clone(), user_1)
            .expect_err("should be impossible to update info by anyone now");
        token
            .update_info(new_info_3.clone(), controller.unwrap())
            .expect_err("should be impossible to update info by anyone now");
        token
            .update_info(new_info_3, user_2)
            .expect_err("should be impossible to update info by anyone now");

        // --- ADD/DELETE LISTENER ---
        let new_listener_1 = OnMoveEventListener {
            filter: OnMoveEventFilter::any(),
            endpoint: random_endpoint(),
        };
        token
            .add_on_move_event_listener(new_listener_1.clone(), user_1)
            .expect_err("should be impossible to add listener by not the controller");
        let listener_id_1 = token
            .add_on_move_event_listener(new_listener_1, controller.unwrap())
            .expect("should be possible to add listener by the controller");
        token
            .delete_on_move_event_listener(listener_id_1, user_1)
            .expect_err("should be impossible to delete listener by not the controller");
        token
            .delete_on_move_event_listener(listener_id_1, controller.unwrap())
            .expect("should be possible to delete listener by the controller");

        token
            .update_event_listeners_controller(Some(user_1), user_1)
            .expect_err("should be impossible to update on_move controller by not the controller");
        token
            .update_event_listeners_controller(Some(user_1), controller.unwrap())
            .expect("should be possible to update on_move controller by the controller");

        let new_listener_2 = OnMoveEventListener {
            filter: OnMoveEventFilter::any(),
            endpoint: random_endpoint(),
        };
        token
            .add_on_move_event_listener(new_listener_2.clone(), controller.unwrap())
            .expect_err("should be impossible to add listener by the old controller");
        let listener_id_2 = token
            .add_on_move_event_listener(new_listener_2, user_1)
            .expect("should be possible to add listener by user_1");
        token
            .delete_on_move_event_listener(listener_id_2, controller.unwrap())
            .expect_err("should be impossible to delete listener by the old controller");
        token
            .delete_on_move_event_listener(listener_id_2, user_1)
            .expect("should be possible to delete listener by user_1");

        token
            .update_event_listeners_controller(None, controller.unwrap())
            .expect_err("should be impossible to update on_move controller by the old controller");
        token
            .update_event_listeners_controller(None, user_1)
            .expect("should be possible to update on_move controller by user_1");

        let new_listener_3 = OnMoveEventListener {
            filter: OnMoveEventFilter::any(),
            endpoint: random_endpoint(),
        };
        token
            .add_on_move_event_listener(new_listener_3.clone(), user_1)
            .expect_err("should be impossible to add listener by anyone");
        token
            .add_on_move_event_listener(new_listener_3.clone(), user_2)
            .expect_err("should be impossible to add listener by anyone");
        token
            .add_on_move_event_listener(new_listener_3, controller.unwrap())
            .expect_err("should be impossible to add listener by anyone");
    }
}
