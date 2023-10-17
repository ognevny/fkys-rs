#[test]
fn hw() {
    let command = std::process::Command::new("sh")
        .arg("./.ci/test.sh")
        .output()
        .expect("failed to execute");
    if !command.status.success() {
        println!(
            "failed with {}\ncode {}",
            String::from_utf8_lossy(&command.stdout),
            command.status.code().unwrap_or(-1)
        );
    } else {
        println!("success! {}", String::from_utf8_lossy(&command.stdout));
    }
}
