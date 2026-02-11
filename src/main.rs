use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use python_cleaner::*;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Show help if no argument or --help
    if args.len() < 2 || args[1] == "--help" {
        println!("Usage: python-cleaner <file.py>");
        println!("Cleans Python files for syntax, git, and file size issues.");
        println!("Example: python-cleaner myfile.py");
        return;
    }

    let file_path = &args[1];
    let path = Path::new(file_path);

    if !path.exists() {
        eprintln!("❌ Error: File '{}' does not exist.", file_path);
        return;
    }

    // Read the Python file
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Error reading file '{}': {}", file_path, e);
            return;
        }
    };

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
            eprintln!("⚠️  Invalid choice.");
            return;
        }
    };

    // Prompt for cleaned file name
    let final_file_name = loop {
        println!("\nEnter the name for the cleaned file (include .py), or press Enter to use default:");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut new_file_name = String::new();
        io::stdin().read_line(&mut new_file_name).unwrap();
        let new_file_name = new_file_name.trim();

        if new_file_name.is_empty() {
            // Default name if user presses Enter
            let file_stem = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy();
            let extension = path
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
    if let Err(e) = fs::write(&final_file_name, cleaned) {
        eprintln!("❌ Failed to write cleaned file '{}': {}", final_file_name, e);
        return;
    }

    println!("\n✅ Cleaning complete.");
    println!("Cleaned file saved as: {}", final_file_name);
}
