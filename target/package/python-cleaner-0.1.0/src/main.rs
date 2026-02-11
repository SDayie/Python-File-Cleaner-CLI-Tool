use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use python_cleaner::*;

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
