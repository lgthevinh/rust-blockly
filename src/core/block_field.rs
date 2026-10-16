use serde::{Deserialize, Serialize};

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
