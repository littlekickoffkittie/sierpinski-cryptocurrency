use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriangleState {
    Genesis,
    Active,
    Subdivided,
    Void,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriangleMetadata {
    pub state: TriangleState,
    pub depth: u32,
    pub parent_id: Option<String>,
    pub children_ids: Vec<String>,
}

impl TriangleMetadata {
    pub fn new(state: TriangleState, depth: u32) -> Self {
        TriangleMetadata {
            state,
            depth,
            parent_id: None,
            children_ids: Vec::new(),
        }
    }
}
