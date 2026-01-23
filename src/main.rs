use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: python-cleaner <file.py>");
        return;
    }

    let file_path = &args[1];
    let content = fs::read_to_string(file_path)
        .expect("Failed to read file");

    println!("What's your main problem?");
    println!("1. Syntax errors");
    println!("2. Git issues");
    println!("3. File size");
    println!("4. Everything");
    print!("Enter choice: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    let cleaned = match choice {
        "1" => clean_syntax(&content),
        "2" => clean_git(&content),
        "3" => clean_size(&content),
        "4" => clean_all(&content),
        _ => {
            eprintln!("Invalid choice.");
            return;
        }
    };

    fs::write(file_path, cleaned).expect("Failed to write file");
    println!("✅ Cleaning complete.");
}

fn clean_all(input: &str) -> String {
    let step1 = tabs_to_spaces(input);
    let step2 = remove_trailing_spaces(&step1);
    let step3 = fix_backslash_space(&step2);
    let step4 = normalize_blank_lines(&step3);
    normalize_line_endings(&step4)
}

fn clean_syntax(input: &str) -> String {
    let step1 = tabs_to_spaces(input);
    let step2 = fix_backslash_space(&step1);
    normalize_line_endings(&step2)
}

fn clean_git(input: &str) -> String {
    let step1 = remove_trailing_spaces(input);
    normalize_blank_lines(&step1)
}

fn clean_size(input: &str) -> String {
    let step1 = remove_trailing_spaces(input);
    normalize_blank_lines(&step1)
}

/* ---------- Core Cleaners ---------- */

fn tabs_to_spaces(input: &str) -> String {
    input.replace("\t", "    ")
}

fn remove_trailing_spaces(input: &str) -> String {
    input.lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}

fn fix_backslash_space(input: &str) -> String {
    input.lines()
        .map(|line| {
            if line.trim_end().ends_with("\\") {
                line.trim_end().to_string()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn normalize_blank_lines(input: &str) -> String {
    let mut result = Vec::new();
    let mut last_blank = false;

    for line in input.lines() {
        let is_blank = line.trim().is_empty();
        if is_blank {
            if !last_blank {
                result.push("");
            }
        } else {
            result.push(line);
        }
        last_blank = is_blank;
    }

    result.join("\n")
}

fn normalize_line_endings(input: &str) -> String {
    input.replace("\r\n", "\n")
}
