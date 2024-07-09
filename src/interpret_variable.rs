use crate::interpret::{Value, VariableMap};

#[derive(Debug)]
pub struct Variable {
    pub name: String,
}

pub fn interpret_variable_expr(variable: &Variable, variable_map: &VariableMap) -> Value {
    match variable_map.get(&variable.name) {
        Some(x) => match &x[..] {
            [val, ..] => val.clone(),
            _ => panic!("Val not found"),
        },
        None => panic!("Undefined variable"),
    }
}
