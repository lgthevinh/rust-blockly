use serde::{Deserialize, Serialize};

/// Block Input: a named value slot that takes a literal or another block (`Block` holds its id).
/// Lets blocks nest, so an expression block can feed its result into a parent block.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BInput {
    pub name: String,
    #[serde(flatten)]
    pub input: BInputType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "input_type", content = "value")]
pub enum BInputType {
    Block(u32),
    Int(i32),
    Double(f64),
    String(String),
    Bool(bool),
}
