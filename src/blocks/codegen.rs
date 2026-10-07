pub trait Generator {
    fn workspace_to_code(ws: &Workspace);
    fn statement_to_code(block: &Block, name: String);
    fn value_to_code(block: &Block, name: String);
    
}