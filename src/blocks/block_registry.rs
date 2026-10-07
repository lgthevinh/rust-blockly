use std::collections::HashMap;

use crate::{blocks::block_def::BlockDef, core::block::Block};

pub struct BlockRegistry {
    hat_block_map: HashMap<String, BlockDef>,
    block_map: HashMap<String, BlockDef>,
}

impl BlockRegistry {
    fn get_block_def(opcode: String) {}
    fn get_hat_block_def(opcode: String) {}
}
