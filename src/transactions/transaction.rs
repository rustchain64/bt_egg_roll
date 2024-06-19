use crate::state::SharedState;

#[derive(Clone, Debug)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub zk_proof: Vec<u8>, // Placeholder for zk-SNARK proof
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u64, zk_proof: Vec<u8>) -> Self {
        Transaction {
            from,
            to,
            amount,
            zk_proof,
        }
    }

    pub fn validate(&self) -> bool {
        // Simulate zk-SNARK proof validation
        self.zk_proof.len() > 0 // In reality, use a zk-SNARK library to validate the proof
    }
}

pub fn process_transaction(state: &SharedState, tx: Transaction) -> Result<(), String> {
    if !tx.validate() {
        return Err("Invalid zk-SNARK proof".into());
    }

    let mut state = state.lock().unwrap();
    state.withdraw(&tx.from, tx.amount)?;
    state.deposit(tx.to, tx.amount);
    Ok(())
}

pub fn process_batch(state: &SharedState, transactions: Vec<Transaction>) {
    for tx in transactions {
        match process_transaction(state, tx) {
            Ok(_) => println!("Transaction processed successfully"),
            Err(e) => println!("Transaction failed: {}", e),
        }
    }
}
