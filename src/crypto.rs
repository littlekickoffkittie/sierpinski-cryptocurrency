use sha3::{Digest, Sha3_256};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FractalHash {
    pub bytes: [u8; 32],
}

impl FractalHash {
    pub fn new(bytes: [u8; 32]) -> Self {
        FractalHash { bytes }
    }

    pub fn from_slice(bytes: &[u8]) -> Self {
        let mut hash = [0u8; 32];
        hash.copy_from_slice(bytes);
        FractalHash::new(hash)
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.bytes)
    }

    pub fn from_hex(hex_str: &str) -> Result<Self, hex::FromHexError> {
        let bytes = hex::decode(hex_str)?;
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&bytes);
        Ok(FractalHash::new(hash))
    }
}

impl fmt::Display for FractalHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

#[derive(Debug, Clone)]
pub struct FractalHasher {
    hasher: Sha3_256,
}

impl FractalHasher {
    pub fn new() -> Self {
        FractalHasher {
            hasher: Sha3_256::new(),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }

    pub fn finalize(self) -> FractalHash {
        let result = self.hasher.finalize();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&result);
        FractalHash::new(bytes)
    }

    pub fn hash_triangle_coordinates(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, depth: u32) -> FractalHash {
        let mut hasher = FractalHasher::new();
        
        let data = format!("{:.10},{:.10},{:.10},{:.10},{:.10},{:.10},{}", 
                          x1, y1, x2, y2, x3, y3, depth);
        
        hasher.update(data.as_bytes());
        hasher.finalize()
    }

    pub fn hash_triangle_vertices(vertices: &[[f64; 2]; 3], depth: u32) -> FractalHash {
        let mut hasher = FractalHasher::new();
        
        for vertex in vertices {
            let data = format!("{:.10},{:.10}", vertex[0], vertex[1]);
            hasher.update(data.as_bytes());
        }
        
        let depth_data = format!("{}", depth);
        hasher.update(depth_data.as_bytes());
        
        hasher.finalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fractal_hash_generation() {
        let hash = FractalHasher::hash_triangle_coordinates(0.0, 0.0, 1.0, 0.0, 0.5, 0.866, 0);
        assert_eq!(hash.bytes.len(), 32);
        assert!(!hash.bytes.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_hash_consistency() {
        let hash1 = FractalHasher::hash_triangle_coordinates(0.0, 0.0, 1.0, 0.0, 0.5, 0.866, 0);
        let hash2 = FractalHasher::hash_triangle_coordinates(0.0, 0.0, 1.0, 0.0, 0.5, 0.866, 0);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_hex_conversion() {
        let hash = FractalHash::new([0x01; 32]);
        let hex_str = hash.to_hex();
        let parsed = FractalHash::from_hex(&hex_str).unwrap();
        assert_eq!(hash, parsed);
    }
}
