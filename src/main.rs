use rust_blockly::{blocks::arduino::arduino, core::Blockly};

fn main() {
    let mut blockly = Blockly::new();
    blockly.define_blocks(arduino());

}
