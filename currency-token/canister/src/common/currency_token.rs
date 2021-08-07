use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_cron::types::TaskId;

use antifragile_currency_token_client::types::{
    ControllerList, Controllers, CurrencyTokenInfo, Error,
};

#[derive(CandidType, Deserialize)]
pub struct CurrencyToken {
    pub balances: HashMap<Principal, u64>,
    pub total_supply: u64,
    pub info: CurrencyTokenInfo,
    pub controllers: ControllerList,
    pub recurrent_mint_tasks: HashSet<TaskId>,
    pub recurrent_transfer_tasks: HashMap<Principal, HashSet<TaskId>>,
}

impl CurrencyToken {
    pub fn mint(&mut self, to: Principal, qty: u64) -> Result<(), Error> {
        if qty == 0 {
            return Err(Error::ZeroQuantity);
        }

        let prev_balance = self.balance_of(&to);
        let new_balance = prev_balance + qty;

        self.total_supply += qty;
        self.balances.insert(to, new_balance);

        Ok(())
    }

    pub fn transfer(&mut self, from: Principal, to: Principal, qty: u64) -> Result<(), Error> {
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

        Ok(())
    }

    pub fn burn(&mut self, from: Principal, qty: u64) -> Result<(), Error> {
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

        Ok(())
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

    pub fn register_recurrent_transfer_task(&mut self, from: Principal, task_id: TaskId) {
        match self.recurrent_transfer_tasks.entry(from) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().insert(task_id);
            }
            Entry::Vacant(entry) => {
                let mut s = HashSet::new();
                s.insert(task_id);

                entry.insert(s);
            }
        };
    }

    pub fn unregister_recurrent_transfer_task(&mut self, from: Principal, task_id: TaskId) -> bool {
        match self.recurrent_transfer_tasks.get_mut(&from) {
            Some(tasks) => tasks.remove(&task_id),
            None => false,
        }
    }

    pub fn get_recurrent_transfer_tasks(&self, from: Principal) -> Vec<TaskId> {
        self.recurrent_transfer_tasks
            .get(&from)
            .map(|t| t.iter().cloned().collect::<Vec<_>>())
            .unwrap_or_default()
    }

    pub fn register_recurrent_mint_task(&mut self, task_id: TaskId) {
        self.recurrent_mint_tasks.insert(task_id);
    }

    pub fn unregister_recurrent_mint_task(&mut self, task_id: TaskId) -> bool {
        self.recurrent_mint_tasks.remove(&task_id)
    }

    pub fn get_recurrent_mint_tasks(&self) -> Vec<TaskId> {
        self.recurrent_mint_tasks.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

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
            recurrent_mint_tasks: HashSet::new(),
            recurrent_transfer_tasks: HashMap::new(),
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

        token.mint(user_1, 100).ok().unwrap();

        assert_eq!(token.total_supply, 100);
        assert_eq!(token.balances.len(), 1);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 100);

        token.mint(controller, 200).ok().unwrap();

        assert_eq!(token.total_supply, 300);
        assert_eq!(token.balances.len(), 2);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 100);
        assert_eq!(token.balances.get(&controller).unwrap().clone(), 200);
    }

    #[test]
    fn burning_works_fine() {
        let (mut token, controller) = create_currency_token();
        let user_1 = random_principal_test();

        token.mint(user_1, 100).ok().unwrap();

        token.burn(user_1, 90).ok().unwrap();

        assert_eq!(token.balances.len(), 1);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 10);
        assert_eq!(token.total_supply, 10);

        token.burn(user_1, 20).err().unwrap();

        token.burn(user_1, 10).ok().unwrap();

        assert!(token.balances.is_empty());
        assert!(token.balances.get(&user_1).is_none());
        assert_eq!(token.total_supply, 0);

        token.burn(user_1, 20).err().unwrap();
    }

    #[test]
    fn transfer_works_fine() {
        let (mut token, controller) = create_currency_token();
        let user_1 = random_principal_test();
        let user_2 = random_principal_test();

        token.mint(user_1, 1000).ok().unwrap();

        token.transfer(user_1, user_2, 100).ok().unwrap();

        assert_eq!(token.balances.len(), 2);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 900);
        assert_eq!(token.balances.get(&user_2).unwrap().clone(), 100);
        assert_eq!(token.total_supply, 1000);

        token.transfer(user_1, user_2, 1000).err().unwrap();

        token.transfer(controller, user_2, 100).err().unwrap();

        token.transfer(user_2, user_1, 100).ok().unwrap();

        assert_eq!(token.balances.len(), 1);
        assert_eq!(token.balances.get(&user_1).unwrap().clone(), 1000);
        assert!(token.balances.get(&user_2).is_none());
        assert_eq!(token.total_supply, 1000);

        token.transfer(user_2, user_1, 1).err().unwrap();

        token.transfer(user_2, user_1, 0).err().unwrap();
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
