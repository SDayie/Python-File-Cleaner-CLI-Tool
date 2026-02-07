# 🐍 Python Cleaner Tool

## Automatically manage whitespace and fix basic Python syntax issues without changing your code logic. 

- Converts tabs to spaces (**prevents `TabError`**)  
- Fixes simple syntax issues like **trailing backslashes at the end of lines**  
- Removes trailing spaces (**for cleaner diffs and commits**)  
- Normalizes blank lines (**improves readability and reduces file size**)  
- Creates a new cleaned file (**without overwriting your original**)  

**Ideal for:** pre-commit hooks, legacy code maintenance, and team collaboration.


## Installation

### **Prerequisites**
- Install [Rust](https://www.rust-lang.org/tools/install) on your system.  
- Ensure `cargo` is available in your terminal.

### ** Download / Clone the Tool from GitHub
Open your terminal (macOS/Linux) or PowerShell (Windows) and run:
git clone https://github.com/SDayie/Python-File-Cleaner-CLI-Tool.git
cd "Python-File-Cleaner-CLI-Tool"
This downloads the tool to your computer and navigates into the project folder.
 
### ** Run the Tool (without installing globally)
macOS / Linux
cargo run -- /path/to/your/python/file.py
Example:
cargo run -- ~/Downloads/dirty_example.py
Windows (PowerShell)
cargo run -- "C:\Users\YourName\Downloads\dirty_example.py"
 
### ** Run the Tool (after installing globally, optional)
You can install the CLI globally (so you can run it from any folder):
cargo install --path .
After installation, simply run:
python-cleaner /path/to/your/python/file.py
On macOS/Linux, the binary is usually in ~/.cargo/bin/python-cleaner
On Windows, in %USERPROFILE%\.cargo\bin\python-cleaner.exe
 
### ** Choose what to clean
After running, the tool will ask:
What's your main problem?
1. Syntax errors
2. Git issues
3. File size
4. Everything
Enter choice:

### ** What each choice does:
Choice	Fixes
1. Syntax errors	Tabs → spaces, trailing spaces after \, normalize line endings
2. Git issues	Trailing spaces, multiple blank lines, consistent formatting for cleaner diffs
3. File size	Reduce unnecessary blank lines and trailing spaces
4. Everything	All of the above

Type a number and press Enter.
 
### ** Enter the new cleaned file name
You will see:
Enter the name for the cleaned file (include .py), or press Enter to use default:
>
•	Type a new name, e.g., my_cleaned.py
•	Or press Enter to use the default: <original>_cleaned.py
The original file is never modified.
 
### ** Confirm Cleaning
Cleaned file saved as: dirty_example_cleaned.py
•	You can now run the cleaned file:
python dirty_example_cleaned.py
 

## Features

### **Syntax Error Prevention**
- **Fixes mixed indentation**: Converts tabs to spaces, preventing `TabError`  
- **Cleans line continuations**: Removes trailing spaces after backslashes (`\`)  

### **Whitespace Cleaning**
- **Trailing space removal**: Eliminates spaces/tabs at line ends (reduces git noise) 
- **Blank line optimization**: Reduces consecutive blank lines to maximum one between blocks  

### 📁 **File Handling**
- **Safe by default**: Creates `filename_cleaned.py` instead of overwriting   
- **Custom output**: Option to specify output filename  

### **User Interface**
- **Interactive mode**: Choose problems to fix (syntax errors/git issues/file size/everything)

### **Git/GitHub Optimization**
- **Cleaner diffs**: Removes whitespace-only changes from git history   



## Limitations

### **No Logic or Semantic Changes**
- **Logic unchanged**: Never modifies your code's functionality or algorithm   
- **No bug fixes**: Runtime errors (`ZeroDivisionError`, `KeyError`, etc.) remain 
- **No refactoring**: Variable names, function structure, and imports stay as-is 

### **Limited Syntax Error Coverage**
- **Only basic whitespace-related syntax**: Fixes `TabError` and removes trailing spaces after line continuation backslashes (`\`)
- **Not a full linter**: Doesn't catch missing parentheses, colons, or complex syntax errors  
- **No code analysis**: Can't detect logical indentation mistakes (wrong blocks) 

### **Basic Formatting Only**
- **Not PEP 8 compliant**: Only fixes basic whitespace, not full Python style   
- **No line length management**: Won't wrap long lines to 79 characters 
- **No import organization**: Imports stay in their original order 
- **No naming fixes**: Variable/function names remain unchanged 

### **File Processing Scope**
- **Single-file focus**: Primarily designed for individual file processing   

### **Git Integration Level**
- **Cleaning only**: Doesn't commit, push, or modify git history   
- **Pre-commit helper**: Best used before commits, not as git replacement   
- **Manual integration**: Requires adding to pre-commit hooks or CI/CD manually   



### **Before & After Code Comparison**

### **Syntax Errors Demo**
### ❌ Before Cleaning
<img width="1277" height="520" alt="syntax_error" src="https://github.com/user-attachments/assets/bb83cddf-2392-42d4-92e2-ca16b2d49ed1" />

 Python fails due to invisible whitespace issues.

Mixed tabs and spaces combined with a broken line continuation cause a TabError / SyntaxError, even though the logic is correct.*

### ✅ After Cleaning
<img width="1277" height="520" alt="syntax_error_cleaned" src="https://github.com/user-attachments/assets/971bc473-6ab2-42c7-b014-f63ccc83e9a6" />

Same logic, valid syntax.
Tabs are converted to spaces and trailing whitespace after the backslash is removed. The script now runs successfully without changing behavior.

### **Git Issues Demo**
### ❌ Before Cleaning
<img width="1277" height="520" alt="git_issues" src="https://github.com/user-attachments/assets/5d1050bb-9c2a-4bb7-aa7b-711ce0b55a36" />

Whitespace noise in version control.
Trailing spaces and excessive blank lines create unnecessary changes in git diffs and clutter commits.
### ✅ After Cleaning
<img width="1277" height="520" alt="git_issues_cleaned" src="https://github.com/user-attachments/assets/8e99c6b3-063f-439a-b1e2-df247ca7ff2d" />

Clean diffs, cleaner commits.
Trailing whitespace is removed and blank lines are normalized, reducing git noise without modifying code logic.
 
### **File Size Demo**
### ❌ Before Cleaning
<img width="1277" height="791" alt="file_size" src="https://github.com/user-attachments/assets/6129e62e-ec4a-4e10-a51a-a0117257b561" />

Unnecessarily long and bloated file.
Excessive blank lines increase file length and reduce readability.
### ✅ After Cleaning
<img width="1277" height="600" alt="file_size_cleaned" src="https://github.com/user-attachments/assets/221dd5d4-0c4b-408d-92d7-40defc617271" />

Compact and readable.
Multiple blank lines are collapsed into a single line, improving readability and reducing file size.

