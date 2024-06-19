use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Address = String;
type Balance = u64;

#[derive(Clone, Debug)]
pub struct Account {
    pub balance: Balance,
}

#[derive(Clone, Debug)]
pub struct RollupState {
    pub accounts: HashMap<Address, Account>,
}

impl RollupState {
    pub fn new() -> Self {
        RollupState {
            accounts: HashMap::new(),
        }
    }

    pub fn deposit(&mut self, address: Address, amount: Balance) {
        let account = self
            .accounts
            .entry(address.clone())
            .or_insert(Account { balance: 0 });
        account.balance += amount;
    }

    pub fn withdraw(&mut self, address: &Address, amount: Balance) -> Result<(), String> {
        if let Some(account) = self.accounts.get_mut(address) {
            if account.balance >= amount {
                account.balance -= amount;
                return Ok(());
            }
        }
        Err("Insufficient balance".into())
    }

    pub fn get_balance(&self, address: &Address) -> Balance {
        self.accounts.get(address).map_or(0, |acc| acc.balance)
    }
}

pub type SharedState = Arc<Mutex<RollupState>>;
