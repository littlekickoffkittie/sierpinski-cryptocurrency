use serde::{Deserialize, Serialize};
use crate::triangle::TriangleState;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionType {
    Create,
    Subdivide,
    UpdateState(TriangleState),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub triangle_id: String,
    pub transaction_type: TransactionType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Transaction {
    pub fn new(triangle_id: String, transaction_type: TransactionType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            triangle_id,
            transaction_type,
            timestamp: chrono::Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::triangle::Triangle;

    #[test]
    fn test_new_transaction() {
        let triangle = Triangle::genesis();
        let transaction = Transaction::new(triangle.id.clone(), TransactionType::Create);

        assert_eq!(transaction.triangle_id, triangle.id);
        assert_eq!(transaction.transaction_type, TransactionType::Create);
    }
}
