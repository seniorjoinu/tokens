use std::collections::HashMap;

use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use union_utils::{TotalVotingPowerUpdateEvent, VotingPowerUpdateEvent};

use antifragile_currency_token_client::events::TokenMoveEvent;
use antifragile_currency_token_client::types::{
    ControllerList, Controllers, CurrencyTokenInfo, Error, Payload,
};

#[derive(CandidType, Deserialize)]
pub struct CurrencyToken {
    pub balances: HashMap<Principal, u64>,
    pub total_supply: u64,
    pub info: CurrencyTokenInfo,
    pub controllers: ControllerList,
}

impl CurrencyToken {
    pub fn mint(
        &mut self,
        to: Principal,
        qty: u64,
        payload: Payload,
    ) -> Result<
        (
            TokenMoveEvent,
            TotalVotingPowerUpdateEvent,
            VotingPowerUpdateEvent,
        ),
        Error,
    > {
        if qty == 0 {
            return Err(Error::ZeroQuantity);
        }

        let prev_balance = self.balance_of(&to);
        let new_balance = prev_balance + qty;

        self.total_supply += qty;
        self.balances.insert(to, new_balance);

        Ok((
            TokenMoveEvent {
                from: None,
                to: Some(to),
                qty,
                payload,
            },
            TotalVotingPowerUpdateEvent {
                new_total_voting_power: self.total_supply,
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
                from: Some(from),
                to: Some(to),
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
    ) -> Result<
        (
            TokenMoveEvent,
            TotalVotingPowerUpdateEvent,
            VotingPowerUpdateEvent,
        ),
        Error,
    > {
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
                from: Some(from),
                to: None,
                qty,
                payload,
            },
            TotalVotingPowerUpdateEvent {
                new_total_voting_power: self.total_supply,
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

    pub fn update_mint_controllers(&mut self, new_mint_controllers: Controllers) -> Controllers {
        let old_controllers = self.controllers.mint_controllers.clone();
        self.controllers.mint_controllers = new_mint_controllers;

        old_controllers
    }

    pub fn update_info_controllers(&mut self, new_info_controllers: Controllers) -> Controllers {
        let old_controllers = self.controllers.info_controllers.clone();
        self.controllers.info_controllers = new_info_controllers;

        old_controllers
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

    use ic_cdk::export::candid::Principal;
    use union_utils::random_principal_test;

    use antifragile_currency_token_client::types::{ControllerList, CurrencyTokenInfo};

    use crate::common::currency_token::CurrencyToken;

    fn magic_blob() -> Vec<u8> {
        vec![1u8, 3u8, 3u8, 7u8]
    }

    fn create_currency_token() -> (CurrencyToken, Principal) {
        let controller = random_principal_test();
        let token = CurrencyToken {
            balances: HashMap::new(),
            total_supply: 0,
            info: CurrencyTokenInfo {
                name: String::from("test"),
                symbol: String::from("TST"),
                decimals: 8,
            },
            controllers: ControllerList::single(Some(controller)),
        };

        (token, controller)
    }

    #[test]
    fn creation_works_fine() {
        let (token, controller) = create_currency_token();

        assert!(token.balances.is_empty());
        assert_eq!(token.total_supply, 0);
        assert!(token.controllers.info_controllers.contains(&controller));
        assert!(token.controllers.mint_controllers.contains(&controller));
        assert_eq!(token.info.name, String::from("test"));
        assert_eq!(token.info.symbol, String::from("TST"));
        assert_eq!(token.info.decimals, 8);
    }

    #[test]
    fn minting_works_right() {
        let (mut token, controller) = create_currency_token();
        let user_1 = random_principal_test();

        let (ev1, ev2, ev3) = token.mint(user_1, 100, None).ok().unwrap();

        assert_eq!(token.total_supply, 100);
        assert_eq!(token.balances.len(), 1);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 100);

        assert_eq!(ev1.from, None);
        assert_eq!(ev1.to, Some(user_1));
        assert_eq!(ev1.qty, 100);
        assert_eq!(ev1.payload, None);

        assert_eq!(ev2.new_total_voting_power, 100);

        assert_eq!(ev3.voter, user_1);
        assert_eq!(ev3.new_voting_power, 100);

        let (ev1, ev2, ev3) = token
            .mint(controller, 200, Some(magic_blob()))
            .ok()
            .unwrap();

        assert_eq!(token.total_supply, 300);
        assert_eq!(token.balances.len(), 2);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 100);
        assert_eq!(token.balances.get(&controller).unwrap().clone(), 200);

        assert_eq!(ev1.from, None);
        assert_eq!(ev1.qty, 200);
        assert_eq!(ev1.to, Some(controller));
        assert_eq!(ev1.payload, Some(magic_blob()));

        assert_eq!(ev2.new_total_voting_power, 300);

        assert_eq!(ev3.voter, controller);
        assert_eq!(ev3.new_voting_power, 200);
    }

    #[test]
    fn burning_works_fine() {
        let (mut token, controller) = create_currency_token();
        let user_1 = random_principal_test();

        token.mint(user_1, 100, None).ok().unwrap();

        let (ev1, ev2, ev3) = token.burn(user_1, 90, None).ok().unwrap();

        assert_eq!(token.balances.len(), 1);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 10);
        assert_eq!(token.total_supply, 10);

        assert_eq!(ev1.from, Some(user_1));
        assert_eq!(ev1.to, None);
        assert_eq!(ev1.qty, 90);
        assert_eq!(ev1.payload, None);

        assert_eq!(ev2.new_total_voting_power, 10);

        assert_eq!(ev3.voter, user_1);
        assert_eq!(ev3.new_voting_power, 10);

        token.burn(user_1, 20, None).err().unwrap();

        let (ev1, ev2, ev3) = token.burn(user_1, 10, None).ok().unwrap();

        assert!(token.balances.is_empty());
        assert!(token.balances.get(&user_1).is_none());
        assert_eq!(token.total_supply, 0);

        assert_eq!(ev1.from, Some(user_1));
        assert_eq!(ev1.to, None);
        assert_eq!(ev1.qty, 10);
        assert_eq!(ev1.payload, None);

        assert_eq!(ev2.new_total_voting_power, 0);

        assert_eq!(ev3.voter, user_1);
        assert_eq!(ev3.new_voting_power, 0);

        token.burn(user_1, 20, None).err().unwrap();
    }

    #[test]
    fn transfer_works_fine() {
        let (mut token, controller) = create_currency_token();
        let user_1 = random_principal_test();
        let user_2 = random_principal_test();

        token.mint(user_1, 1000, None).ok().unwrap();

        let (ev1, ev2, ev3) = token.transfer(user_1, user_2, 100, None).ok().unwrap();

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

        token.transfer(user_1, user_2, 1000, None).err().unwrap();

        token.transfer(controller, user_2, 100, None).err().unwrap();

        let (ev1, ev2, ev3) = token.transfer(user_2, user_1, 100, None).ok().unwrap();

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

        token.transfer(user_2, user_1, 1, None).err().unwrap();

        token.transfer(user_2, user_1, 0, None).err().unwrap();
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
