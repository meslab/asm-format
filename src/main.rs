use std::env;
use std::fs;

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

    for line in input.lines() {
        let prefix = match starts_with_whitespace(&line) {
            true => "    ",
            false => "",
        };
        let first_word = line.trim().split_whitespace().next().unwrap_or("");
        let remaining_part = line
            .trim()
            .split_whitespace()
            .skip(1)
            .collect::<Vec<&str>>()
            .join(" ");

        match remaining_part.trim().is_empty() {
            true => formatted_lines.push(format!("{}{}", prefix, first_word)),
            false => {
                if prefix.is_empty()
                    && first_word
                        .trim_start_matches('_')
                        .chars()
                        .next()
                        .expect("No first word!")
                        .is_uppercase()
                {
                    formatted_lines.push(format!("{}{:<11} {}", prefix, first_word, remaining_part))
                } else {
                    formatted_lines.push(format!("{}{:<7} {}", prefix, first_word, remaining_part))
                }
            }
        }
    }

    formatted_lines.join("\n")
}

fn starts_with_whitespace(line: &str) -> bool {
    !line.is_empty() && line.chars().next().unwrap().is_whitespace()
}
