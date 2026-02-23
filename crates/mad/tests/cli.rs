use predicates::prelude::*;

fn mad() -> assert_cmd::Command {
    assert_cmd::cargo::cargo_bin_cmd!("mad")
}

#[test]
fn renders_markdown_file() {
    let dir = tempfile::tempdir().expect("should create tempdir");
    let file = dir.path().join("test.md");
    std::fs::write(&file, "# Hello\n\nThis is **bold** text.\n").expect("should write file");

    mad()
        .arg(file.to_str().expect("path should be valid"))
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello"))
        .stdout(predicate::str::contains("bold"));
}

#[test]
fn renders_from_stdin() {
    mad()
        .write_stdin("# Title\n\nSome text.\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Title"))
        .stdout(predicate::str::contains("Some text."));
}

#[test]
fn no_highlight_flag() {
    let dir = tempfile::tempdir().expect("should create tempdir");
    let file = dir.path().join("test.md");
    std::fs::write(&file, "```rust\nfn main() {}\n```\n").expect("should write file");

    mad()
        .arg("--no-highlight")
        .arg(file.to_str().expect("path should be valid"))
        .assert()
        .success()
        .stdout(predicate::str::contains("fn main()"));
}

#[test]
fn width_flag() {
    mad()
        .args(["--width", "40"])
        .write_stdin("This is a long line of text that should get wrapped at the specified column width for display purposes.\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("\n"));
}

#[test]
fn missing_file_errors() {
    mad()
        .arg("/nonexistent/file.md")
        .assert()
        .failure()
        .stderr(predicate::str::contains("mad:"));
}

#[test]
fn version_flag() {
    mad()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.0.1"));
}

#[test]
fn man_subcommand_outputs_roff() {
    mad()
        .arg("man")
        .assert()
        .success()
        .stdout(predicate::str::contains(".TH"))
        .stdout(predicate::str::contains("mad"));
}

#[test]
fn code_block_with_syntax_highlighting() {
    let dir = tempfile::tempdir().expect("should create tempdir");
    let file = dir.path().join("test.md");
    std::fs::write(
        &file,
        "# Code Example\n\n```toml\n[package]\nname = \"test\"\n```\n",
    )
    .expect("should write file");

    mad()
        .arg(file.to_str().expect("path should be valid"))
        .assert()
        .success()
        .stdout(predicate::str::contains("package"))
        .stdout(predicate::str::contains("\x1b["));
}
