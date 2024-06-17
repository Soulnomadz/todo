use assert_cmd::Command;
use predicates::prelude::*;
use anyhow::Result;

#[test]
fn missing_args() -> Result<()>{
    Command::cargo_bin("todo")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));

    Ok(())
}

#[test]
fn add_with_more_args() -> Result<()> {
    Command::cargo_bin("todo")?
        .args(["add", "hello", "world"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error"));

    Ok(())
}

#[test]
fn add_item() -> Result<()> {
    let item = "hello";

    let expected = format!("Item added successfully: .*:{item}");

    Command::cargo_bin("todo")?
        .args(["add", item])
        .assert()
        .success()
        .stdout(predicate::str::is_match(expected)?);

    Ok(())
}

