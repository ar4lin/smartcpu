use std::process::Command;

pub fn run_command(cmd: &str) -> String{
    let output: String = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap()
        .to_string();

    if output.is_empty() {
        "N/A".to_string()
    } else {
        output
    }
}