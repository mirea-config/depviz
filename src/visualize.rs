use std::io::Error;

#[cfg(target_os = "linux")]
pub fn form_uml_str<'a>(package: &'a str, dependencies: &Vec<String>) -> String {
    let start = "@startuml depviz";
    let end = "@enduml";
    
    let body = dependencies
        .iter()
        .map(|f| format!("{} -> {}", package, f))
        .collect::<Vec<String>>()
        .join("\n");

    format!("{}\n{}\n{}", start, body, end)
}

#[cfg(target_os = "linux")]
pub fn visualize(bin_path: &str, src_path: &str) -> Result<(), Error> {
    use std::process::Command;

    let output = Command::new(bin_path)
        .arg(src_path)
        .output()?;

    if output.status.success() {
        println!("uml graph ")
    } else {

    }
}
