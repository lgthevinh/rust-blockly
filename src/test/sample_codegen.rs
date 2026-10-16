use crate::core::bdef::BlockDef;
use crate::core::bdef_arg::Arg;
use crate::core::block_field::{BField, BFieldType};
use crate::core::block_input::{BInput, BInputType};
use std::collections::HashMap;

use crate::core::{block::Block, codegen::Codegen, workspace::Workspace};

pub struct ArduinoCodegen {
    includes: HashMap<String, String>,
    setups: HashMap<String, String>,
}

impl ArduinoCodegen {
    fn new() -> Self {
        Self {
            includes: HashMap::new(),
            setups: HashMap::new(),
        }
    }
}

impl Codegen for ArduinoCodegen {
    fn convert_block_to_code(&self, block: &Block) -> String {
        match block.op_code.as_str() {
            "serial_print" => {
                format!("Serial.print()\n")
            }
            "digital_write" => {
                format!("digital_write()\n")
            }
            _ => {
                format!("// Missing code generator for {}\n", block.op_code)
            }
        }
    }
}

pub fn test_block() -> Block {
    Block {
        id: 1,
        op_code: "digital_write".into(),
        is_hat: false,
        input: Some(vec![BInput {
            name: "PIN".into(),
            input: BInputType::Int(13),
        }]),
        field: None,
        prev_bid: None,
        next_bid: None,
    }
}

#[test]
pub fn test() {
    let arduino_codegen: ArduinoCodegen = ArduinoCodegen::new();
    let block = test_block();
    let code = arduino_codegen.convert_block_to_code(&block);
    print!("{}", code)
}
