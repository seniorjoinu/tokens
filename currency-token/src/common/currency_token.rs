use std::collections::HashMap;

use ic_cdk::export::candid::{CandidType, Deserialize, Principal};

use crate::common::api::{TokenMoveEvent, VotingPowerUpdateEvent};
use crate::common::types::{Account, Controllers, CurrencyTokenInfo, Error, Payload};

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct CurrencyToken {
    pub balances: HashMap<Principal, u64>,
    pub total_supply: u64,
    pub info: CurrencyTokenInfo,
    pub controllers: Controllers,
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
            },
        ))
    }

    pub fn transfer(
        &mut self,
        from: Principal,
        to: Principal,
        qty: u64,
        payload: Payload,
    ) -> Result<
        (
            TokenMoveEvent,
            VotingPowerUpdateEvent,
            VotingPowerUpdateEvent,
        ),
        Error,
    > {
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
            },
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
            },
        ))
    }

    pub fn update_info(&mut self, new_info: CurrencyTokenInfo) -> CurrencyTokenInfo {
        let old_info = self.info.clone();
        self.info = new_info;

        old_info
    }

    pub fn update_mint_controller(&mut self, new_mint_controller: Account) -> Account {
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

    pub fn update_info_controller(&mut self, new_info_controller: Account) -> Account {
        let old_controller = self.controllers.info_controller;
        self.controllers.info_controller = new_info_controller;

        old_controller
    }

    pub fn balance_of(&self, account_owner: &Principal) -> u64 {
        match self.balances.get(account_owner) {
            None => 0,
            Some(b) => *b,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::time::{SystemTime, UNIX_EPOCH};

    use ic_cdk::export::candid::Principal;

    use crate::common::currency_token::CurrencyToken;
    use crate::common::types::{Account, Controllers, CurrencyTokenInfo};

    fn magic_blob() -> Vec<u8> {
        vec![1u8, 3u8, 3u8, 7u8]
    }

    fn random_principal() -> Principal {
        Principal::from_slice(
            &SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                .to_be_bytes(),
        )
    }

    fn create_currency_token() -> (CurrencyToken, Account) {
        let controller = Some(random_principal());
        let token = CurrencyToken {
            balances: HashMap::new(),
            total_supply: 0,
            info: CurrencyTokenInfo {
                name: String::from("test"),
                symbol: String::from("TST"),
                decimals: 8,
            },
            controllers: Controllers::single(controller),
        };

        (token, controller)
    }

    #[test]
    fn creation_works_fine() {
        let (token, controller) = create_currency_token();

        assert!(token.balances.is_empty());
        assert_eq!(token.total_supply, 0);
        assert_eq!(token.controllers.event_listeners_controller, controller);
        assert_eq!(token.controllers.info_controller, controller);
        assert_eq!(token.controllers.mint_controller, controller);
        assert_eq!(token.info.name, String::from("test"));
        assert_eq!(token.info.symbol, String::from("TST"));
        assert_eq!(token.info.decimals, 8);
    }

    #[test]
    fn minting_works_right() {
        let (mut token, controller) = create_currency_token();
        let user_1 = random_principal();

        let (ev1, ev2) = token.mint(user_1, 100, None).expect("mint 1 should work");

        assert_eq!(token.total_supply, 100);
        assert_eq!(token.balances.len(), 1);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 100);

        assert_eq!(ev1.from, None);
        assert_eq!(ev1.to, Some(user_1));
        assert_eq!(ev1.qty, 100);
        assert_eq!(ev1.payload, None);

        assert_eq!(ev2.voter, user_1);
        assert_eq!(ev2.new_voting_power, 100);

        let (ev1, ev2) = token
            .mint(controller.unwrap(), 200, Some(magic_blob()))
            .expect("mint 2 should work");

        assert_eq!(token.total_supply, 300);
        assert_eq!(token.balances.len(), 2);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 100);
        assert_eq!(
            token.balances.get(&controller.unwrap()).unwrap().clone(),
            200
        );

        assert_eq!(ev1.from, None);
        assert_eq!(ev1.qty, 200);
        assert_eq!(ev1.to, controller);
        assert_eq!(ev1.payload, Some(magic_blob()));

        assert_eq!(ev2.voter, controller.unwrap());
        assert_eq!(ev2.new_voting_power, 200);
    }

    #[test]
    fn burning_works_fine() {
        let (mut token, controller) = create_currency_token();
        let user_1 = random_principal();

        token.mint(user_1, 100, None).expect("mint 1 should work");

        let (ev1, ev2) = token.burn(user_1, 90, None).expect("burn 1 should work");

        assert_eq!(token.balances.len(), 1);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 10);
        assert_eq!(token.total_supply, 10);

        assert_eq!(ev1.from, Some(user_1));
        assert_eq!(ev1.to, None);
        assert_eq!(ev1.qty, 90);
        assert_eq!(ev1.payload, None);

        assert_eq!(ev2.voter, user_1);
        assert_eq!(ev2.new_voting_power, 10);

        token
            .burn(user_1, 20, None)
            .expect_err("overburn shouldn't work");

        let (ev1, ev2) = token.burn(user_1, 10, None).expect("burn 2 should work");

        assert!(token.balances.is_empty());
        assert!(token.balances.get(&user_1).is_none());
        assert_eq!(token.total_supply, 0);

        assert_eq!(ev1.from, Some(user_1));
        assert_eq!(ev1.to, None);
        assert_eq!(ev1.qty, 10);
        assert_eq!(ev1.payload, None);

        assert_eq!(ev2.voter, user_1);
        assert_eq!(ev2.new_voting_power, 0);

        token
            .burn(user_1, 20, None)
            .expect_err("overburn shouldn't work");
    }

    #[test]
    fn transfer_works_fine() {
        let (mut token, controller) = create_currency_token();
        let user_1 = random_principal();
        let user_2 = random_principal();

        token.mint(user_1, 1000, None).expect("mint 1 should work");

        let (ev1, ev2, ev3) = token
            .transfer(user_1, user_2, 100, None)
            .expect("transfer 1 should work");

        assert_eq!(token.balances.len(), 2);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 900);
        assert_eq!(token.balances.get(&user_2).unwrap().clone(), 100);
        assert_eq!(token.total_supply, 1000);

        assert_eq!(ev1.from, Some(user_1));
        assert_eq!(ev1.to, Some(user_2));
        assert_eq!(ev1.qty, 100);
        assert_eq!(ev1.payload, None);

        assert_eq!(ev2.voter, user_1);
        assert_eq!(ev2.new_voting_power, 900);

        assert_eq!(ev3.voter, user_2);
        assert_eq!(ev3.new_voting_power, 100);

        token
            .transfer(user_1, user_2, 1000, None)
            .expect_err("overtrasnfer 1 should fail");

        token
            .transfer(controller.unwrap(), user_2, 100, None)
            .expect_err("overtransfer 2 should fail");

        let (ev1, ev2, ev3) = token
            .transfer(user_2, user_1, 100, None)
            .expect("transfer 2 should work");

        assert_eq!(token.balances.len(), 1);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 1000);
        assert!(token.balances.get(&user_2).is_none());
        assert_eq!(token.total_supply, 1000);

        assert_eq!(ev1.from, Some(user_2));
        assert_eq!(ev1.to, Some(user_1));
        assert_eq!(ev1.qty, 100);
        assert_eq!(ev1.payload, None);

        assert_eq!(ev2.voter, user_2);
        assert_eq!(ev2.new_voting_power, 0);

        assert_eq!(ev3.voter, user_1);
        assert_eq!(ev3.new_voting_power, 1000);

        token
            .transfer(user_2, user_1, 1, None)
            .expect_err("overtransfer 3 should fail");

        token
            .transfer(user_2, user_1, 0, None)
            .expect_err("transfer of zero tokens should fail");
    }

    #[test]
    fn info_update_works_well() {
        let (mut token, controller) = create_currency_token();

        let new_info_1 = CurrencyTokenInfo {
            name: String::from("name 1"),
            symbol: String::from("NME1"),
            decimals: 9,
        };
        token.update_info(new_info_1);

        assert_eq!(token.info.name, String::from("name 1"));
        assert_eq!(token.info.symbol, String::from("NME1"));
        assert_eq!(token.info.decimals, 9);

        let new_info_2 = CurrencyTokenInfo {
            name: String::from("name 2"),
            symbol: String::from("NME2"),
            decimals: 2,
        };
        token.update_info(new_info_2);

        assert_eq!(token.info.name, String::from("name 2"));
        assert_eq!(token.info.symbol, String::from("NME2"));
        assert_eq!(token.info.decimals, 2);
    }
}
