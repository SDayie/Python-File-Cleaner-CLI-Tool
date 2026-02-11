/* ---------- Cleaning Modes ---------- */

pub fn clean_all(input: &str) -> String {
    let step1 = tabs_to_spaces(input);
    let step2 = remove_trailing_spaces(&step1);
    let step3 = fix_backslash_space(&step2);
    let step4 = normalize_blank_lines(&step3);
    normalize_line_endings(&step4)
}

pub fn clean_syntax(input: &str) -> String {
    let step1 = tabs_to_spaces(input);
    let step2 = fix_backslash_space(&step1);
    normalize_line_endings(&step2)
}

pub fn clean_git(input: &str) -> String {
    let step1 = remove_trailing_spaces(input);
    normalize_blank_lines(&step1)
}

pub fn clean_size(input: &str) -> String {
    let step1 = remove_trailing_spaces(input);
    normalize_blank_lines(&step1)
}

/* ---------- Core Cleaning Functions ---------- */

pub fn tabs_to_spaces(input: &str) -> String {
    input.replace("\t", "    ")
}

pub fn remove_trailing_spaces(input: &str) -> String {
    input.lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn fix_backslash_space(input: &str) -> String {
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

pub fn normalize_blank_lines(input: &str) -> String {
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

pub fn normalize_line_endings(input: &str) -> String {
    input.replace("\r\n", "\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------- Core Function Tests ----------

    #[test]
    fn test_tabs_to_spaces() {
        let input = "\tprint('hello')";
        let expected = "    print('hello')";
        assert_eq!(tabs_to_spaces(input), expected);
    }

    #[test]
    fn test_remove_trailing_spaces() {
        let input = "hello   \nworld   ";
        let expected = "hello\nworld";
        assert_eq!(remove_trailing_spaces(input), expected);
    }

    #[test]
    fn test_fix_backslash_space() {
        let input = "print('hi') \\   \nnext";
        let expected = "print('hi') \\\nnext";
        assert_eq!(fix_backslash_space(input), expected);
    }

    #[test]
    fn test_normalize_blank_lines() {
        let input = "line1\n\n\nline2\n\n";
        let expected = "line1\n\nline2\n";
        assert_eq!(normalize_blank_lines(input), expected);
    }

    #[test]
    fn test_normalize_line_endings() {
        let input = "line1\r\nline2\r\n";
        let expected = "line1\nline2\n";
        assert_eq!(normalize_line_endings(input), expected);
    }

    // ---------- Wrapper / Combined Function Tests ----------

    #[test]
    fn test_clean_all() {
        let input = "\tprint('hi') \\   \r\n\r\n";
        let expected = "    print('hi') \\";
        assert_eq!(clean_all(input), expected);
    }

    #[test]
    fn test_clean_syntax() {
        let input = "\tprint('hi') \\   \r\n";
        let expected = "    print('hi') \\";
        assert_eq!(clean_syntax(input), expected);
    }

    #[test]
    fn test_clean_git() {
        let input = "line with spaces   \n\n\nline2";
        let expected = "line with spaces\n\nline2";
        assert_eq!(clean_git(input), expected);
    }

    #[test]
    fn test_clean_size() {
        let input = "line with spaces   \n\n\nline2";
        let expected = "line with spaces\n\nline2";
        assert_eq!(clean_size(input), expected);
    }

    // ---------- Edge Case Tests ----------

    #[test]
    fn test_empty_input() {
        let input = "";
        let expected = "";
        assert_eq!(clean_all(input), expected);
    }

    #[test]
    fn test_only_blank_lines() {
        let input = "\n\n\n";
        let expected = "";
        assert_eq!(clean_all(input), expected);
    }

    #[test]
    fn test_tabs_and_trailing_spaces() {
        let input = "\tline1   \n\tline2  ";
        let expected = "    line1\n    line2";
        assert_eq!(clean_all(input), expected);
    }

    #[test]
    fn test_backslash_at_end() {
        let input = "line with backslash \\   ";
        let expected = "line with backslash \\";
        assert_eq!(clean_all(input), expected);
    }
}
