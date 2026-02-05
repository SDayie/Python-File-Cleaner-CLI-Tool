use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: python-cleaner <file.py>");
        return;
    }

    let file_path = &args[1];

    // Read the original Python file
    let content = fs::read_to_string(file_path)
        .expect("Failed to read file");

    // Ask the user what they want to clean
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

    // Clean according to the user's choice
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

    // Prompt for cleaned file name with a loop to ensure input works
    let final_file_name = loop {
        println!("\nEnter the name for the cleaned file (include .py), or press Enter to use default:");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut new_file_name = String::new();
        io::stdin().read_line(&mut new_file_name).unwrap();
        let new_file_name = new_file_name.trim();

        if new_file_name.is_empty() {
            // Default name if user presses Enter
            let original_path = Path::new(file_path);
            let file_stem = original_path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy();
            let extension = original_path
                .extension()
                .unwrap_or_default()
                .to_string_lossy();
            break format!("{}_cleaned.{}", file_stem, extension);
        } else if new_file_name.ends_with(".py") {
            break new_file_name.to_string();
        } else {
            println!("⚠️  File name must end with .py. Try again.");
        }
    };

    // Write cleaned content to the new file
    fs::write(&final_file_name, cleaned).expect("Failed to write cleaned file");

    println!("\n✅ Cleaning complete.");
    println!("Cleaned file saved as: {}", final_file_name);
}

/* ---------- Cleaning Modes ---------- */

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

/* ---------- Core Cleaning Functions ---------- */

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
