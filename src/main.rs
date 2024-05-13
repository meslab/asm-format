use std::env;
use std::fs;
use std::io::{self, Write};
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            std::process::exit(1);
        }
    };

    let formatted_contents = format_assembly(&contents);

    match fs::write(filename, formatted_contents) {
        Ok(()) => println!("Assembly code formatted successfully!"),
        Err(err) => eprintln!("Error writing file: {}", err),
    };
}

fn format_assembly(input: &str) -> String {
    let mut formatted_lines = Vec::new();
    let re_spaces = Regex::new(r" {4}").unwrap();
    let re_spaces_words = Regex::new(r"([^\s,]+) +").unwrap();

    for line in input.lines() {
        let line = re_spaces.replace_all(&line, "\t");
        let line = re_spaces_words.replace_all(&line, |caps: &regex::Captures| {
            let first_word = caps.get(1).unwrap().as_str();
            if first_word.ends_with(',') && caps[0].contains(',') {
                caps[0].to_string()
            } else {
                format!("{}\t", first_word)
            }
        });
        formatted_lines.push(line.to_string());
    }

    formatted_lines.join("\n")
}
