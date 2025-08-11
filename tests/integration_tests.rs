//! Integration tests for the Sierpinski Crypto system

#[cfg(test)]
mod integration_tests {
    use sierpinski_crypto::mining::{MiningAlgorithm, MiningChallenge, ChallengeType};
    use sierpinski_crypto::triangle::genesis_implementation::GenesisTriad;

    #[test]
    fn test_full_mining_workflow() {
        // Create root genesis triangle
        let root = GenesisTriad::root();
        assert!(root.is_equilateral());
        
        // Create mining challenge
        let challenge = MiningChallenge::new(
            root.calculate_hash(),
            4,
            ChallengeType::SubdivisionDepth { target_depth: 1 },
        );
        
        // Create mining algorithm
        let algorithm = MiningAlgorithm::new(4, challenge);
        
        // Mine a new block
        let result = algorithm.mine(&root);
        assert!(result.is_ok());
        
        let mining_result = result.unwrap();
        assert_eq!(mining_result.solution.depth, 1);
        assert!(mining_result.solution.is_equilateral());
    }

    #[test]
    fn test_triangle_subdivision_chain() {
        let mut current = GenesisTriad::root();
        
        // Mine 3 levels deep
        for depth in 1..=3 {
            let challenge = MiningChallenge::new(
                current.calculate_hash(),
                4,
                ChallengeType::SubdivisionDepth { target_depth: depth },
            );
            
            let algorithm = MiningAlgorithm::new(4, challenge);
            let result = algorithm.mine(&current).unwrap();
            
            current = result.solution;
            assert_eq!(current.depth, depth);
        }
    }

    #[test]
    fn test_geometric_consistency() {
        let root = GenesisTriad::root();
        let children = root.subdivide();
        
        // Verify all children are equilateral
        for child in &children {
            assert!(child.is_equilateral());
        }
        
        // Verify area relationships
        let root_area = root.area();
        let child_areas: Vec<u128> = children.iter().map(|c| c.area()).collect();
        
        // Children should have smaller areas
        for child_area in child_areas {
            assert!(child_area < root_area);
        }
    }
}
