use assert_cmd::cargo;
use predicates::prelude::*;
use std::fs;

fn write_temp_file(name: &str, content: &str) {
    fs::write(name, content).unwrap();
}

fn remove_temp_file(name: &str) {
    if std::path::Path::new(name).exists() {
        fs::remove_file(name).unwrap();
    }
}

#[test]
fn test_cli_clean_syntax() {
    let temp_file = "temp_syntax.py";
    write_temp_file(temp_file, "\tprint('hi') \\   \n");

    let mut cmd = cargo::cargo_bin_cmd!("python-cleaner");
    cmd.arg(temp_file)
        .write_stdin("1\n\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Cleaning complete."));

    remove_temp_file(temp_file);
    remove_temp_file("temp_syntax_cleaned.py");
}

#[test]
fn test_cli_clean_git() {
    let temp_file = "temp_git.py";
    write_temp_file(temp_file, "line with spaces   \n\n\nline2");

    let mut cmd = cargo::cargo_bin_cmd!("python-cleaner");
    cmd.arg(temp_file)
        .write_stdin("2\n\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Cleaning complete."));

    remove_temp_file(temp_file);
    remove_temp_file("temp_git_cleaned.py");
}

#[test]
fn test_cli_clean_size() {
    let temp_file = "temp_size.py";
    write_temp_file(temp_file, "line with spaces   \n\n\nline2");

    let mut cmd = cargo::cargo_bin_cmd!("python-cleaner");
    cmd.arg(temp_file)
        .write_stdin("3\n\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Cleaning complete."));

    remove_temp_file(temp_file);
    remove_temp_file("temp_size_cleaned.py");
}

#[test]
fn test_cli_clean_all() {
    let temp_file = "temp_all.py";
    write_temp_file(temp_file, "\tprint('hi') \\   \n\n");

    let mut cmd = cargo::cargo_bin_cmd!("python-cleaner");
    cmd.arg(temp_file)
        .write_stdin("4\n\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Cleaning complete."));

    remove_temp_file(temp_file);
    remove_temp_file("temp_all_cleaned.py");
}
