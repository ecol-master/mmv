use assert_cmd::Command;
use std::fs::File;
use tempdir::TempDir;

fn format_expected(test_name: &str, error_msg: &str) -> String {
    format!("TEST: {}, error: {}", test_name, error_msg)
}

#[test]
fn test_basic() {
    let source_dir = TempDir::new("test_dir").expect("failed to create test_dir");
    let file = "file-1.txt";
    let source_pattern = "file-*.txt";
    let target_pattern = "file-#1-v1.txt";

    let file_path = source_dir.path().join(file);
    File::create(&file_path).expect(&format!("failed create: {}", &file));

    let source_dir_path = source_dir.path().to_str().unwrap();
    let mut cmd = Command::cargo_bin("mmv").expect("failed run mmv binary");

    let file_path_arg = format!("{}/{}", source_dir_path, source_pattern);
    let pattern_arg = format!("{}/{}", source_dir_path, target_pattern);

    let expected_from = format!("{}/{}", source_dir_path, file);
    let expected_to = format!("{}/{}", source_dir_path, "file-1-v1.txt");
    let expected_output = format!("{} -> {}\n", expected_from, expected_to);

    cmd.arg(file_path_arg)
        .arg(pattern_arg)
        .assert()
        .stdout(expected_output)
        .success();
}

#[test]
fn test_run_with_no_files() {
    let test_name = "mmv with no files test";

    let source_dir = TempDir::new("test_dir").expect("failed to create test_dir");
    let source_pattern = "file-*.txt";
    let target_pattern = "file-#1-v1.txt";

    let source_dir_path = source_dir.path().to_str().unwrap();
    let mut cmd =
        Command::cargo_bin("mmv").expect(&format_expected(&test_name, "failed run mmv binary"));

    let file_path_arg = format!("{}/{}", source_dir_path, source_pattern);
    let pattern_arg = format!("{}/{}", source_dir_path, target_pattern);

    let expected_err = format!("mmv: Files for pattern '{}' not found\n", source_pattern);
    cmd.arg(file_path_arg)
        .arg(pattern_arg)
        .assert()
        .failure()
        .stderr(expected_err);
}

#[test]
fn test_invalid_source_directory() {
    let source_dir_path = "some_unexist_dir";
    let source_pattern = "file-*.txt";
    let target_pattern = "file-#1-v1.txt";
    let target_dir_path = "unexists_dir";

    let source_dir_path = source_dir_path;
    let mut cmd = Command::cargo_bin("mmv").expect("failed run mmv binary");

    let file_path_arg = format!("{}/{}", source_dir_path, source_pattern);
    let pattern_arg = format!("{}/{}", target_dir_path, target_pattern);

    let expected_err = "mmv: Directory `some_unexist_dir` no found\n";

    cmd.arg(file_path_arg)
        .arg(pattern_arg)
        .assert()
        .failure()
        .stderr(expected_err);
}

#[test]
fn test_invalid_target_pattern() {
    let source_dir = TempDir::new("test_dir").expect("failed to create test_dir");
    let file = "file-1.txt";
    let source_pattern = "file-*.txt";
    let target_pattern = "file-#2-v1.txt";

    let file_path = source_dir.path().join(file);
    File::create(&file_path).expect(&format!("failed create: {}", &file));

    let source_dir_path = source_dir.path().to_str().unwrap();
    let mut cmd = Command::cargo_bin("mmv").expect("failed run mmv binary");

    let file_path_arg = format!("{}/{}", source_dir_path, source_pattern);
    let pattern_arg = format!("{}/{}", source_dir_path, target_pattern);

    let expected_err = "mmv: Invalid target path: position #2 not exist in source path\n";

    cmd.arg(file_path_arg)
        .arg(pattern_arg)
        .assert()
        .failure()
        .stderr(expected_err);
}

#[test]
fn test_invalid_target_directory() {
    let source_dir = TempDir::new("test_dir").expect("failed to create test_dir");
    let file = "file-1.txt";
    let source_pattern = "file-*.txt";
    let target_pattern = "file-#1-v1.txt";

    let file_path = source_dir.path().join(file);
    File::create(&file_path).expect(&format!("failed create: {}", &file));

    let target_dir_path = "unexists_dir";

    let source_dir_path = source_dir.path().to_str().unwrap();
    let mut cmd = Command::cargo_bin("mmv").expect("failed run mmv binary");

    let file_path_arg = format!("{}/{}", source_dir_path, source_pattern);
    let pattern_arg = format!("{}/{}", target_dir_path, target_pattern);

    let expected_err = "mmv: Directory `unexists_dir` no found\n";

    cmd.arg(file_path_arg)
        .arg(pattern_arg)
        .assert()
        .failure()
        .stderr(expected_err);
}
