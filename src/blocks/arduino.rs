use crate::core::{block_def::BlockDef};

pub fn arduino() -> Vec<BlockDef> {
    vec![
        BlockDef {
            opcode: "arduino_init",
            msg: "when arduino start",
            args: vec![],
            is_hat: true,
        }, 
        BlockDef {
            opcode: "arduino_loop",
            msg: "when loop",
            args: vec![],
            is_hat: true,
        }
    ]
}
