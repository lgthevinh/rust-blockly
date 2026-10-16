use crate::core::block::Block;
use crate::core::var::Var;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    pub blocks: Vec<Block>,
    pub vars: HashMap<String, Var>,
}
