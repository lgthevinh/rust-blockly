use crate::core::{block::Block, workspace::Workspace};

pub trait Codegen {
    fn convert_ws_to_code(&self, ws: &Workspace) -> String {
        let mut code = String::new();
        for block in ws.blocks.iter() {
            let block_code = self.convert_block_to_code(block);
            code += &block_code
        }
        
        code
    }

    fn convert_block_to_code(&self, block: &Block) -> String;
}