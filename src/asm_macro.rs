use crate::datum::{Datum};

pub fn disassemble_macro(code: &str, _data: &[Datum]) -> Option<Vec<String>> {
    let contents = code.trim_start().split('\t').collect::<Vec<&str>>();
    if contents.len() > 0 {
        let name = contents[0];
        match name {
            "nop" => Some(nop()),
            _ => None,
        }
    } else {
        panic!("Invalid instruction {}.", code)
    }
}

fn nop() -> Vec<String> {
    let mut result: Vec<String> = vec![];
    result.push(format!("add\tr0,r0,r0"));
    result
}
