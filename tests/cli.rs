use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs


#[test]
fn extension_is_wrong() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp.child("empty.txt");
    file.touch().unwrap();

    let mut cmd = Command::cargo_bin("mvx")?;
    cmd.arg(file.path()).arg("-r").arg(".bak");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("empty.txt does not end with .bak"));

    Ok(())
}

#[test]
fn file_is_missing() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp.child("empty.txt");
    file.touch().unwrap();

    let mut cmd = Command::cargo_bin("mvx")?;
    cmd.arg("phantom.txt").arg("-r").arg(".txt");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("phantom.txt does not exist."));

    Ok(())
}

#[test]
fn add_extension_ok() -> Result<(), Box<dyn std::error::Error>> {
    let temp_a = assert_fs::TempDir::new().unwrap();
    let file_a = temp_a.child("empty_a.txt");
    file_a.touch().unwrap();
    let temp_b = assert_fs::TempDir::new().unwrap();
    let file_b = temp_b.child("empty_b.txt");
    file_b.touch().unwrap();

    let mut cmd = Command::cargo_bin("mvx")?;
    cmd.arg(file_a.path())
        .arg(file_b.path())
        .arg("-a")
        .arg(".bak");
    cmd.assert().success();

    assert!(temp_a.child("empty_a.txt.bak").path().exists());
    assert!(temp_b.child("empty_b.txt.bak").path().exists());

    Ok(())
}

#[test]
fn remove_extension_ok() -> Result<(), Box<dyn std::error::Error>> {
    let temp_a = assert_fs::TempDir::new().unwrap();
    let file_a = temp_a.child("empty_a.txt.bak");
    file_a.touch().unwrap();
    let temp_b = assert_fs::TempDir::new().unwrap();
    let file_b = temp_b.child("empty_b.txt.bak");
    file_b.touch().unwrap();

    let mut cmd = Command::cargo_bin("mvx")?;
    cmd.arg(file_a.path())
        .arg(file_b.path())
        .arg("-r")
        .arg(".bak");
    cmd.assert().success();

    assert!(temp_a.child("empty_a.txt").path().exists());
    assert!(temp_b.child("empty_b.txt").path().exists());

    Ok(())
}

#[test]
fn mv_extension_ok() -> Result<(), Box<dyn std::error::Error>> {
    let temp_a = assert_fs::TempDir::new().unwrap();
    let file_a = temp_a.child("empty_a.txt");
    file_a.touch().unwrap();
    let temp_b = assert_fs::TempDir::new().unwrap();
    let file_b = temp_b.child("empty_b.txt");
    file_b.touch().unwrap();

    let mut cmd = Command::cargo_bin("mvx")?;
    cmd.arg(file_a.path())
        .arg(file_b.path())
        .arg("-r")
        .arg(".txt")
        .arg("-a")
        .arg(".md");
    cmd.assert().success();

    assert!(temp_a.child("empty_a.md").path().exists());
    assert!(temp_b.child("empty_b.md").path().exists());

    Ok(())
}
