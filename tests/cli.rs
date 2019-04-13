use assert_cmd::prelude::*;
use dirs;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use regex::Regex;
use std::fs;
use std::process::Command;

#[test]
fn test_add() {
    let first: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let second: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mut cmd = Command::cargo_bin("vole").unwrap();
    let output = cmd.arg("add").arg(&first).arg(&second).output().unwrap();

    assert!(output.status.success());
    assert_eq!(output.stderr.len(), 0);
    assert_eq!(output.stdout.len(), 0);

    let mut path = dirs::home_dir().unwrap();
    path.push(".vole");
    path.push("cards.txt");
    let path = path;

    let cards_content = fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = cards_content.lines().rev().take(1).collect();

    assert_eq!(lines.len(), 1);
    let re = Regex::new(r"^[a-z0-9]{16}$").unwrap();

    let parts: Vec<&str> = lines[0].split("\t").collect();
    assert_eq!(parts.len(), 3);
    assert!(re.is_match(parts[0]));
    assert_eq!(parts[1], first);
    assert_eq!(parts[2], second);
}

#[test]
fn test_biadd() {
    let first: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let second: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mut cmd = Command::cargo_bin("vole").unwrap();
    let output = cmd
        .arg("add")
        .arg("-b")
        .arg(&first)
        .arg(&second)
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(output.stderr.len(), 0);
    assert_eq!(output.stdout.len(), 0);

    let mut path = dirs::home_dir().unwrap();
    path.push(".vole");
    path.push("cards.txt");
    let path = path;

    let cards_content = fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = cards_content.lines().rev().take(2).collect();

    assert_eq!(lines.len(), 2);
    let re = Regex::new(r"^[a-z0-9]{16}$").unwrap();

    let parts: Vec<&str> = lines[0].split("\t").collect();
    assert_eq!(parts.len(), 3);
    assert!(re.is_match(parts[0]));
    assert_eq!(parts[1], second);
    assert_eq!(parts[2], first);

    let parts: Vec<&str> = lines[1].split("\t").collect();
    assert_eq!(parts.len(), 3);
    assert!(re.is_match(parts[0]));
    assert_eq!(parts[1], first);
    assert_eq!(parts[2], second);
}

#[test]
fn test_find() {
    Command::cargo_bin("vole")
        .unwrap()
        .arg("add")
        .arg("this čeština")
        .arg("will")
        .output()
        .unwrap();
    Command::cargo_bin("vole")
        .unwrap()
        .arg("add")
        .arg("this wont")
        .arg("be matched češ")
        .output()
        .unwrap();
    Command::cargo_bin("vole")
        .unwrap()
        .arg("add")
        .arg("and this will also be")
        .arg("řč čeština matched")
        .output()
        .unwrap();

    let output = Command::cargo_bin("vole")
        .unwrap()
        .arg("find")
        .arg("^.*čeština")
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(output.stderr.len(), 0);

    let output = String::from_utf8(output.stdout).unwrap();
    let mut lines: Vec<&str> = output.lines().collect();
    lines.reverse();
    assert!(lines.len() >= 2);

    let re = Regex::new(r"^[a-z0-9]{16}$").unwrap();

    let parts: Vec<&str> = lines[1].split("\t").collect();
    assert_eq!(parts.len(), 3);
    assert!(re.is_match(parts[0]));
    assert_eq!(parts[1], "this čeština");
    assert_eq!(parts[2], "will");

    let parts: Vec<&str> = lines[0].split("\t").collect();
    assert_eq!(parts.len(), 3);
    assert!(re.is_match(parts[0]));
    assert_eq!(parts[1], "and this will also be");
    assert_eq!(parts[2], "řč čeština matched");
}
