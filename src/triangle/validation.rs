use crate::triangle::genesis::GenesisTriad;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    #[error("Triangle is not equilateral")]
    NotEquilateral,

    #[error("Invalid geometric proof")]
    InvalidProof,

    #[error("Parent-child relationship is invalid")]
    InvalidHierarchy,
}

pub struct TriangleValidator;

impl TriangleValidator {
    pub fn validate(triad: &GenesisTriad) -> Result<(), ValidationError> {
        if !triad.is_equilateral() {
            return Err(ValidationError::NotEquilateral);
        }

        // TODO: Implement more validation checks
        // - Proof validation
        // - Hierarchy validation

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::triangle::genesis::{Point, GenesisTriad};

    #[test]
    fn test_equilateral_validation() {
        let center = Point::new(0.0, 0.0);
        let triad = GenesisTriad::new(center, 2.0);
        assert_eq!(TriangleValidator::validate(&triad), Ok(()));
    }

    #[test]
    fn test_non_equilateral_validation() {
        let vertices = [
            Point::new(0.0, 1.0),
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
        ];
        let triad = GenesisTriad {
            vertices,
            area: 0.5,
            depth: 0,
            id: "test".to_string(),
        };
        assert_eq!(TriangleValidator::validate(&triad), Err(ValidationError::NotEquilateral));
    }
}
