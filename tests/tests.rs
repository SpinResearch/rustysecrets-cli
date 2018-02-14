extern crate colored;
use colored::*;

extern crate tempdir;
use tempdir::TempDir;

use std::str;
use std::process::Command;

#[test]
fn raw() {
    run(&["--raw"]);
}

#[test]
fn wrapped() {
    run(&[]);
}

fn run(raw: &[&str]) {
    let secret_path = "tests/resources/secret.txt";
    let secret = include_str!("resources/secret.txt");
    let n = 10;
    let k = 7;

    let tmp = TempDir::new("rustysecrets").unwrap();

    let output_path = tmp.path().to_string_lossy().as_ref().to_string();

    let split_out = Command::new("target/debug/rustysecrets")
        .arg("split")
        .arg(secret_path)
        .args(raw)
        .args(&["-o", &output_path])
        .args(&["-k", &k.to_string()])
        .args(&["-n", &n.to_string()])
        .output()
        .unwrap();

    assert_eq!(str::from_utf8(&split_out.stdout).unwrap(), "");

    assert_eq!(
        str::from_utf8(&split_out.stderr).unwrap(),
        &format!(
            "{} Wrote {} shares to '{}'\n",
            "success:".green().bold(),
            n,
            output_path
        )
    );

    let shares = (0..7)
        .map(|i| format!("{}/share_{}", output_path, i))
        .collect::<Vec<_>>();

    let recover_out = Command::new("target/debug/rustysecrets")
        .arg("recover")
        .args(raw)
        .args(&shares)
        .output()
        .unwrap();

    assert_eq!(str::from_utf8(&recover_out.stdout).unwrap(), secret);
    assert_eq!(str::from_utf8(&recover_out.stderr).unwrap(), "");
}
