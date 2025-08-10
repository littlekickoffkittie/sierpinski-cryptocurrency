use crate::transaction::Transaction;

pub struct Network {
    pub transactions: Vec<Transaction>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
        }
    }

    pub fn broadcast(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::{Transaction, TransactionType};
    // TODO: The Triangle struct is not yet defined.
    // use crate::triangle::Triangle;

    // #[test]
    // fn test_broadcast() {
    //     let mut network = Network::new();
    //     let triangle = Triangle::genesis();
    //     let transaction = Transaction::new(triangle.id.clone(), TransactionType::Create);

    //     network.broadcast(transaction);

    //     assert_eq!(network.transactions.len(), 1);
    // }
}
*/
