use crate::core::block_input::BInputType;

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
