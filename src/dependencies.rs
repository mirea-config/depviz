use std::process::Command;

#[cfg(target_os = "linux")]
pub fn get_dependencies(package: &str) -> Vec<String> {
    let output = Command::new("apt-cache")
        .arg("depends")
        .arg(package)
        .output()
        .expect(format!("failed to get {package} package dependencies").as_str());

    let output_str = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");
    output_str
        .lines()
        .filter(|line| line.starts_with("  Depends:"))
        .map(|line| line.replace("  Depends:", "").trim().to_string())
        .collect()
}
