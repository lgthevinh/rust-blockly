use std::{collections::HashMap, sync::OnceLock};

use crate::core::block_def::BlockDef;

pub mod block;
pub mod block_def;
pub mod codegen;
pub mod var;
pub mod workspace;


#[derive(Default)]
pub struct Blockly {
    blockdef_registry: HashMap<&'static str, BlockDef>,
}

impl Blockly {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define_block(&mut self, blockdef: BlockDef) {
        self.blockdef_registry.insert(blockdef.opcode, blockdef);
    }

    pub fn define_blocks(&mut self, block_defs: Vec<BlockDef>) {
        for blockdef in block_defs {
            self.define_block(blockdef);
        }
    }

    pub fn is_hat_block(&self, opcode: &str) -> Option<bool> {
        self.blockdef_registry.get(opcode).map(|d| d.is_hat)
    }
}
