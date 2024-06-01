use serde::{Deserialize, Serialize};

/// 成果物
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Deliverable {
    pub name: String,
    pub path: String,
}
