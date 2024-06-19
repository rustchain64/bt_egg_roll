use bitcoin_rollup::state::RollupState;
use bitcoin_rollup::transactions::{process_batch, Transaction};
use std::sync::{Arc, Mutex};

fn main() {
    let state = Arc::new(Mutex::new(RollupState::new()));
    let tx1 = Transaction::new("Alice".into(), "Bob".into(), 50, vec![1, 2, 3]);
    let tx2 = Transaction::new("Bob".into(), "Alice".into(), 30, vec![1, 2, 3]);

    {
        let mut state = state.lock().unwrap();
        state.deposit("Alice".into(), 100);
        state.deposit("Bob".into(), 50);
    }

    let transactions = vec![tx1, tx2];
    process_batch(&state, transactions);

    let state = state.lock().unwrap();
    println!("Alice's balance: {}", state.get_balance(&"Alice".into()));
    println!("Bob's balance: {}", state.get_balance(&"Bob".into()));
}
