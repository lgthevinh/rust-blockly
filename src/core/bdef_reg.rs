use std::collections::HashMap;

use crate::core::bdef::BlockDef;

pub struct BlockRegistry {
    block_map: HashMap<String, BlockDef>,
}

impl BlockRegistry {
    fn get_block_def(&self, opcode: &str) -> Option<&BlockDef> {
        self.block_map.get(opcode)
    }

    fn add_block_def(&mut self, block_def: BlockDef) {
        self.block_map.insert(block_def.opcode.clone(), block_def);
    }

    fn is_hat(&self, opcode: &str) -> bool {
        match self.block_map.get(opcode) {
            Some(block) => block.is_hat,
            None => return false,
        }
    }
}
