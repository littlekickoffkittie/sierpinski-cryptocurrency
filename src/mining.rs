use crate::triangle::genesis::GenesisTriad;
use crate::triangle::state::{TriangleMetadata, TriangleState};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MiningChallenge {
    pub triangle_id: String,
    pub target_hash: [u8; 32],
    pub difficulty: u32,
    pub nonce: u64,
}

impl MiningChallenge {
    pub fn new(triangle_id: String, difficulty: u32) -> Self {
        let target_hash = Self::calculate_target_hash(difficulty);
        MiningChallenge {
            triangle_id,
            target_hash,
            difficulty,
            nonce: 0,
        }
    }

    pub fn calculate_target_hash(difficulty: u32) -> [u8; 32] {
        let mut target = [0u8; 32];
        let leading_zeros = difficulty as usize;
        
        // Create target with leading zeros
        for i in 0..leading_zeros {
            if i < 32 {
                target[i] = 0;
            }
        }
        
        // Set remaining bytes to 0xFF for easier comparison
        for i in leading_zeros..32 {
            target[i] = 0xFF;
        }
        
        target
    }

    pub fn hash_input(&self, triangle_data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.triangle_id);
        hasher.update(triangle_data);
        hasher.update(self.nonce.to_le_bytes());
        hasher.finalize().into()
    }

    pub fn is_valid_solution(&self, hash: &[u8; 32]) -> bool {
        hash <= &self.target_hash
    }

    pub fn increment_nonce(&mut self) {
        self.nonce += 1;
    }
}

#[derive(Debug, Clone)]
pub struct Miner {
    pub address: String,
    pub hash_rate: u64,
    pub triangles_mined: Vec<String>,
}

impl Miner {
    pub fn new(address: String) -> Self {
        Miner {
            address,
            hash_rate: 0,
            triangles_mined: Vec::new(),
        }
    }

    pub fn new_miner(address: String) -> Self {
        Miner {
            address,
            hash_rate: 0,
            triangles_mined: Vec::new(),
        }
    }

    pub fn add_miner(&mut self, miner: Miner) {
        self.miners.insert(miner.address.clone(), miner);
    }

    pub fn create_challenge(&mut self, triangle_id: String, difficulty: u32) -> Self {
        let challenge = MiningChallenge::new(triangle_id, difficulty);
        MiningChallenge {
            triangle_id,
            target_hash,
            difficulty,
            nonce: 0,
        }
    }

    pub fn create_challenge(&mut self, triangle_id: String, difficulty: u32) -> Self {
        let challenge = MiningChallenge::new(triangle_id, difficulty);
        MiningChallenge {
            triangle_id,
            target_hash,
            difficulty,
            nonce: 0,
        }
    }

    pub fn create_challenge(&mut self, triangle_id: String, difficulty: u32) -> Self {
        let challenge = MiningChallenge::new(triangle_id, difficulty);
        MiningChallenge {
            triangle_id,
            target_hash,
            difficulty,
            nonce: 0,
        }
    }

    pub fn create_challenge(&mut self, triangle_id: String, difficulty: u32) -> Self {
        let challenge = MiningChallenge::new(triangle_id, difficulty);
        MiningChallenge {
            triangle_id,
            target_hash,
            difficulty,
            nonce: 0,
        }
    }

    pub fn create_challenge(&mut self, triangle_id: String, difficulty: u32) -> Self {
        let challenge = MiningChallenge::new(triangle_id, difficulty);
        MiningChallenge {
            triangle_id,
            target_hash,
            difficulty,
            nonce: 0,
        }
    }

    pub fn create_challenge(&mut self, triangle_id: String, difficulty: u32) -> Self {
        let challenge = MiningChallenge::new(triangle_id, difficulty);
        MiningChallenge {
            triangle_id,
            target_hash,
            difficulty,
            nonce: 0,
        }
    }

    pub fn create_challenge(&mut self, triangle_id: String, difficulty: u32) -> Self {
        let challenge = MiningChallenge::new(triangle_id, difficulty);
        MiningChallenge {
            triangle_id,
            target_hash,
            difficulty,
            nonce: 0,
        }
    }

    pub fn create_challenge(&mut self, triangle_id: String, difficulty: u32) -> Self {
        let challenge = MiningChallenge::new(triangle_id, difficulty);
        MiningChallenge {
            triangle_id,
            target_hash,
            difficulty,
            nonce: 0,
        }
    }

    pub fn create_challenge(&mut self, triangle_id: String, difficulty: u32) -> Self {
        let challenge = MiningChallenge::new(triangle_id, difficulty);
        MiningChallenge {
            triangle_id,
            target_hash,
            difficulty,
            nonce: 0,
        }
    }

    pub fn create_challenge(&mut self, triangle_id: String, difficulty: u32) -> Self {
        let challenge = MiningChallenge::new(triangle_id, difficulty);
        MiningChallenge {
            triangle_id,
            target_hash,
            difficulty,
            nonce: 0,
        }
    }

    pub fn create_challenge(&mut self, triangle_id: String, difficulty: u32) -> Self {
        let challenge = MiningChallenge::new(triangle_id, difficulty);
        MiningChallenge {
            triangle_id,
            target_hash,
            difficulty,
            nonce: 0,
        }
    }

    pub fn create_challenge(&mut self, triangle_id: String, difficulty: u32) -> Self {
        let challenge = MiningChallenge::new(triangle_id, difficulty);
        MiningChallenge {
            triangle_id,
            target_hash,
            difficulty,
            nonce: 0,
        }
    }

    pub fn create_challenge(&mut self, triangle_id: String, difficulty: u32) -> Self {
        let challenge = MiningChallenge::new(triangle_id, difficulty);
        MiningChallenge {
            triangle_id,
            target_hash,
            difficulty,
            nonce: 0,
        }
    }

    pub fn create_challenge(&mut self, triangle_id: String, difficulty: u32) -> Self {
        let challenge = MiningChallenge::new(triangle_id, difficulty);
        MiningChallenge {
            triangle_id,
            target_hash,
            difficulty,
            nonce: 0,
        }
    }

    pub fn create_challenge(&mut self, triangle_id: String], difficulty: u32) -> Self {
        let challenge = MiningChallenge::new(triangle_id, difficulty);
        MiningChallenge {
            triangle_id,
            target_hash,
            difficulty,
            nonce: 0,
        }
    }

    pub fn create_challenge(&mut self, triangle_id: String, difficulty: u32) -> Self {
        let challenge = MiningChallenge::new(triangle_id, difficulty);
        MiningChallenge {
            triangle_id,
            target_hash,
            difficulty,
            nonce: 0,
        }
    }

    pub fn create_challenge(&mut self, triangle_id: String, difficulty: u32) -> Self {
        let challenge = MiningChallenge::new(triangle_id, difficulty);
        MiningChallenge {
