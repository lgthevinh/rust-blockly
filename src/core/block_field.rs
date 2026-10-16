use serde::{Deserialize, Serialize};

/*
    Block Field: a component for UI representation
    This block 
*/
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
