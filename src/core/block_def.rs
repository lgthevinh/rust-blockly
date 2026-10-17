use crate::core::block::{BField, BFieldType, BInput, BInputType, Block};

#[derive(Debug, PartialEq)]
pub enum Arg {
    FieldDropdown {
        name: String,
        options: Vec<(String, String)>,
    },
    FieldText {
        name: String,
        default: String,
    },
    FieldNumber {
        name: String,
        default: f64,
    },
    FieldCheckbox {
        name: String,
        default: bool,
    },
    FieldVariable {
        name: String,
    },
    InputValue {
        name: String,
        default: Option<BInputType>,
    },
    InputStatement {
        name: String,
    },
    InputDummy,
}

#[derive(Debug, PartialEq)]
pub struct BlockDef {
    pub opcode: &'static str,
    pub msg: &'static str,
    pub args: Vec<Arg>,
    pub is_hat: bool,
}

impl BlockDef {
    pub fn to_block(&self, id: u32, prev_bid: Option<u32>, next_bid: Option<u32>) -> Block {
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
            op_code: self.opcode.to_string(),
            is_hat: self.is_hat,
            input: (!input.is_empty()).then_some(input),
            field: (!field.is_empty()).then_some(field),
            prev_bid: prev_bid,
            next_bid: next_bid,
        }
    }

    fn new_field(name: &str, field: BFieldType) -> BField {
        BField {
            name: name.into(),
            field,
        }
    }
}
