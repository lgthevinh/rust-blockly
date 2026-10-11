use crate::core::{
    bdef_arg::Arg,
    block::Block,
    block_field::{BField, BFieldType},
    block_input::BInput,
};

#[derive(Debug, PartialEq)]
pub struct BlockDef {
    pub opcode: String,
    pub msg: String,
    pub args: Vec<Arg>,
    pub is_hat: bool,
}

impl BlockDef {
    pub fn to_block(&self, id: u32) -> Block {
        let mut input = Vec::new();
        let mut field = Vec::new();

        for arg in &self.args {
            match arg {
                Arg::FieldText { name, default } => {
                    field.push(BlockDef::new_field(name, BFieldType::Text(default.clone())))
                }
                Arg::FieldNumber { name, default } => {
                    field.push(BlockDef::new_field(name, BFieldType::Number(*default)))
                }
                Arg::FieldCheckbox { name, default } => {
                    field.push(BlockDef::new_field(name, BFieldType::Checkbox(*default)))
                }
                Arg::FieldDropdown { name, options } => {
                    if let Some((_, value)) = options.first() {
                        field.push(BlockDef::new_field(
                            name,
                            BFieldType::Dropdown(value.clone()),
                        ));
                    }
                }
                Arg::InputValue {
                    name,
                    default: Some(value),
                } => input.push(BInput {
                    name: name.clone(),
                    input: value.clone(),
                }),
                Arg::FieldVariable { .. }
                | Arg::InputValue { default: None, .. }
                | Arg::InputStatement { .. }
                | Arg::InputDummy => {}
            }
        }

        Block {
            id,
            op_code: self.opcode.clone(),
            is_hat: self.is_hat,
            input: (!input.is_empty()).then_some(input),
            field: (!field.is_empty()).then_some(field),
            prev_bid: None,
            next_bid: None,
        }
    }

    fn new_field(name: &str, field: BFieldType) -> BField {
        BField {
            name: name.into(),
            field,
        }
    }
}
