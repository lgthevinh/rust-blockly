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

/// Block Field: a named value the user edits directly on the block (dropdown, text, checkbox, ...).
/// Unlike an input, a field never holds another block. It is a fixed setting of the block itself.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BField {
    pub name: String,
    #[serde(flatten)]
    pub field: BFieldType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "field_type", content = "value")]
pub enum BFieldType {
    Dropdown(String),
    Text(String),
    Number(f64),
    Checkbox(bool),
    Variable(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub id: u32,
    pub op_code: String,
    pub is_hat: bool,

    pub input: Option<Vec<BInput>>,
    pub field: Option<Vec<BField>>,

    pub prev_bid: Option<u32>,
    pub next_bid: Option<u32>,
}
