#![cfg(feature = "cli")]

const EXE: &str = env!("CARGO_BIN_EXE_fkysoxide");
const ROOT: &str = env!("CARGO_MANIFEST_DIR");

use std::process::Command;

#[test]
fn hello_world() {
    let hw = Command::new(EXE).arg(format!("{ROOT}/test_scripts/hw.fkys")).output().unwrap();

    assert_eq!(hw.stdout, b"Hello, world!");
}

#[test]
fn nums() {
    let nums = Command::new(EXE).arg(format!("{ROOT}/test_scripts/nums.fkys")).output().unwrap();

    assert_eq!(nums.stdout, b"46 31 44");
}

#[test]
fn utf8() {
    let utf = Command::new(EXE).arg(format!("{ROOT}/test_scripts/utf.fkys")).output().unwrap();

    assert_eq!(utf.stdout, "я".as_bytes());
}
