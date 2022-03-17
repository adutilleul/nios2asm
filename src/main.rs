use std::env;
use std::fs::File;
use std::io::Write;

mod constants;
mod datum;
mod instruction;
mod label;
mod line;
mod asm_macro;
mod section;
mod text;
mod utils;

use crate::constants::{TEXT_SECTION_MIN_ADDRESS, DATA_SECTION_MIN_ADDRESS, WORD_SIZE};
use crate::datum::{extract_data_from_lines, Datum};
use crate::label::{get_addressed_labels, is_label, resolve_labels, Label};
use crate::line::{compose_lines, Line};
use crate::asm_macro::disassemble_macro;
use crate::section::{resolve_section, Section};
use crate::text::{get_text_from_code, Text};
use crate::utils::{convert_string_to_int};
fn main() {
    println!("nios2assembler-rs (version 1.0.0)");

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("./{} [input_file] [output_file] (text_min_address) (data_min_address)", &args[0]);
        return;
    }

    let mut text_start_address = TEXT_SECTION_MIN_ADDRESS;
    let mut data_start_address = DATA_SECTION_MIN_ADDRESS;

    if args.len() >= 5 {
        text_start_address = convert_string_to_int(&args[3]);
        data_start_address = convert_string_to_int(&args[4]);
    }

    let input_filepath = &args[1];
    let output_filepath = &args[2];
    let mut input_file = File::open(input_filepath).expect("Failed to read input file.");

    let lines = compose_lines(&mut input_file);

    let data = extract_data_from_lines(&lines);
    let codes = extract_codes(&lines, &data);
    let labels = get_addressed_labels(&lines, &codes, text_start_address);
    println!("[+] Disassembling instructions ...");
    let texts = disassemble_instructions(&data, &labels, &codes);
    println!("[+] Writing output assembly file.");
    write_output(output_filepath, &data, &texts, text_start_address, data_start_address);

    println!("[+] Done!");
}

fn extract_codes(lines: &[Line], data: &[Datum]) -> Vec<String> {
    lines
        .iter()
        .filter(|line| {
            line.section == Section::TEXT && resolve_section(line.text.as_ref().unwrap()).is_none()
        })
        .flat_map(|line| {
            if !is_label(&line.text.as_ref().unwrap()) {
                if let Some(pseudo_instruction_codes) =
                disassemble_macro(&line.text.as_ref().unwrap(), &data)
                {
                    pseudo_instruction_codes
                } else {
                    vec![line.text.clone().unwrap().trim_start().to_string()]
                }
            } else {
                vec![line.text.clone().unwrap()]
            }
        })
        .collect()
}

fn disassemble_instructions(data: &[Datum], labels: &[Label], codes: &[String]) -> Vec<Text> {
    let mut current_address = TEXT_SECTION_MIN_ADDRESS;
    codes
        .iter()
        .filter_map(|code| {
            if resolve_labels(&code).is_none() {
                let text = get_text_from_code(&code, current_address, &data, &labels);
                current_address += WORD_SIZE;
                Some(text)
            } else {
                None
            }
        })
        .collect()
}

fn write_output(filepath: &str, data: &[Datum], texts: &[Text], text_start_address: i32, data_start_address: i32) {
    let text_section_size = texts.len() as i32 * WORD_SIZE;

    assert!(text_start_address + text_section_size < data_start_address, "The .text section overlap the .data section!");
    let mut result = vec![format!("v2.0 raw\n{}*0 ", text_start_address)];
    result.extend(texts.iter().map(|text| format!("{} ", text.to_hex())));
    result.push(format!("\n{}*0 ", data_start_address - text_section_size));
    result.extend(data.iter().map(|datum| format!("{} ", datum.to_hex())));
    let mut file = File::create(filepath).expect("Failed to crate output file.");

    write!(file, "{}", result.join("").trim_end()).expect("Failed to write output file.");
}
