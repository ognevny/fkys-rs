#[test]
fn hw() {
    let stat = std::process::Command::new("sh")
        .arg("./.ci/test.sh")
        .status()
        .expect("failed to execute");
    
    assert!(stat.success())
}
