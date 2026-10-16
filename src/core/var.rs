use serde::{Deserialize, Serialize};
use crate::core::block_input::BInputType;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Var {
    pub name: String,
    pub value: BInputType,
}
