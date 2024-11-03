use std::collections::HashMap;
use ic_cdk_macros::{init, update, query};

#[derive(Default)]
struct Token {
    balances: HashMap<String, u64>,
}

impl Token {
    fn send(&mut self, from: &str, to: &str, amount: u64) -> Result<(), String> {
        let sender_balance = self.balances.entry(from.to_string()).or_insert(0);
        if *sender_balance < amount {
            return Err("Insufficient balance".to_string());
        }
        *sender_balance -= amount;
        let receiver_balance = self.balances.entry(to.to_string()).or_insert(0);
        *receiver_balance += amount;
        Ok(())
    }

    fn receive(&mut self, to: &str, amount: u64) {
        let receiver_balance = self.balances.entry(to.to_string()).or_insert(0);
        *receiver_balance += amount;
    }

    fn get_balance(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }
}

thread_local! {
    static TOKEN: std::cell::RefCell<Token> = std::cell::RefCell::new(Token::default());
}

#[init]
fn init() {
    // Initialize the token with some default values or leave it empty for simplicity
}

#[update]
fn send(from: String, to: String, amount: u64) -> Result<(), String> {
    TOKEN.with(|t| t.borrow_mut().send(&from, &to, amount))
}

#[update]
fn receive(to: String, amount: u64) {
    TOKEN.with(|t| t.borrow_mut().receive(&to, amount));
}

#[query]
fn get_balance(address: String) -> u64 {
    TOKEN.with(|t| t.borrow().get_balance(&address))
}
