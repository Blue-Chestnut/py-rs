use py_rs::PY;

/// Defines the type of input and its intial fields
#[derive(PY)]
#[py(tag = "input_type")]
pub enum InputType {
    Text,
    Expression,
    Number {
        min: Option<isize>,
        max: Option<isize>,
    },
    Dropdown {
        options: Vec<(String, String)>,
    },
}

#[derive(PY)]
#[py(tag = "type")]
pub enum InputFieldElement {
    Label {
        text: String,
    },
    Input {
        #[py(flatten)]
        input: InputType,
        name: Option<String>,
        placeholder: Option<String>,
        default: Option<String>,
    },
}

#[derive(PY)]
#[py(export, export_to = "complex_flattened_type/")]
pub struct InputField {
    #[py(flatten)]
    r#type: InputFieldElement,
}

#[test]
fn complex_flattened_type() {
    assert_eq!(
        InputFieldElement::decl(),
        r#"type InputFieldElement = { "type": "Label", text: string, } | { "type": "Input", name: string | null, placeholder: string | null, default: string | null, } & ({ "input_type": "Text" } | { "input_type": "Expression" } | { "input_type": "Number", min: number | null, max: number | null, } | { "input_type": "Dropdown", options: Array<[string, string]>, });"#
    );
    assert_eq!(
        InputField::decl(),
        r#"type InputField = { "type": "Label", text: string, } | { "type": "Input", name: string | null, placeholder: string | null, default: string | null, } & ({ "input_type": "Text" } | { "input_type": "Expression" } | { "input_type": "Number", min: number | null, max: number | null, } | { "input_type": "Dropdown", options: Array<[string, string]>, });"#
    )
}
