use std::fs::File;
use std::io::{Error, ErrorKind, Write};
use uuid::Uuid;
use std::process::{Output, Command};
use std::os::unix::process::ExitStatusExt;
use std::collections::HashSet;

pub struct Depviz<'a> {
    path_to_binary: &'a str,
    output_path: &'a str,
}

impl<'a> Depviz<'a> {
    pub fn new(path_to_binary: &'a str, output_path: &'a str) -> Depviz<'a> {
        return Depviz{
            path_to_binary: &path_to_binary,
            output_path: &output_path
        }
    }

    #[cfg(target_os = "linux")]
    pub fn visualize(&self, package: &'a str, maxdepth: u8, output_format: &'a str) -> Result<(), Error> {
        let mut dependencies: Vec<(String, String)> = vec![];

        self.collect_dependencies(&package, &mut dependencies, maxdepth);

        if dependencies.len() == 0 {
            return Err(Error::new(ErrorKind::Other, format!("no dependencies found for '{package}'")));
        }

        let (id, src_path) = self.write_uml_file(package, &dependencies)?;

        let output_file = format!("{}{}_{}.{}", &self.output_path, &package, &id, &output_format);

        let output = Command::new(&self.path_to_binary)
            .arg(format!("-t{}", &output_format))
            .arg(&src_path)
            .arg("-o")
            .arg(&output_file)
            .output()?;

        if output.status.success() {
            Ok(println!("uml graph file successfully created at '{}'", &output_file))
        } else {
            let err = String::from_utf8_lossy(&output.stderr);
            Err(Error::new(ErrorKind::InvalidInput, err))
        }
    }

    #[cfg(target_os = "linux")]
    fn collect_dependencies(&self, package: &str, deps: &mut Vec<(String, String)>, depth: u8) {
        let output = Command::new("apt-cache")
            .arg("depends")
            .arg(package)
            .output()
            .unwrap_or_else(|_| {
                Output { 
                    status: ExitStatusExt::from_raw(1), 
                    stdout: vec![], 
                    stderr: vec![] 
                }
            });

        let output_str = String::from_utf8(output.stdout).expect("invalid UTF-8 output");

        let curr_deps: Vec<(String, String)> = output_str
            .lines()
            .filter(|line| line.starts_with("  Depends:"))
            .map(|line| (package.to_string(), line.replace("  Depends:", "").trim().to_string()))
            .collect();

        match curr_deps.len() {
            0 => return,
            _ => {
                match depth {
                    0 => {
                        deps.extend(curr_deps);
                        return
                    }
                    _ => {
                        for dep in curr_deps {
                            deps.push(dep.clone());
                            self.collect_dependencies(dep.1.as_str(), deps, depth-1);
                        }
                    }
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    // Функция возвращает кортеж вида (uuid, path), где uuid - айди созданного файла .pu, а path - путь к нему 
    fn write_uml_file(&self, package: &str, dependencies: &Vec<(String, String)>) -> Result<(String, String), Error> {
        let start = format!("@startuml {package}");
        let end = "@enduml";

        let body = self.form_uml_body(&dependencies);

        println!("here is body: \n{}", body);

        let id = Uuid::new_v4().to_string();

        let path = format!("{}{}_{}.pu",  &self.output_path, &package, id);

        let mut file = File::create(&path)?;
        file.write_all(format!("{}\n{}\n{}", start, body, end).as_bytes())?;
        file.flush()?;
    
        Ok((id, path))
    }

    fn form_uml_body(&self, dependencies: &Vec<(String, String)>) -> String {
        let mut visited_deps: HashSet<String> = HashSet::new();
        let mut unique_deps: HashSet<(String, String)> = HashSet::new();

        let body = dependencies
            .iter()
            .filter_map(|dep| {
                if unique_deps.contains(&(dep.1.clone(), dep.0.clone())) {
                    return None; 
                }

                let d = format!("\"{}\" -> \"{}\"", dep.0, dep.1);
                if !visited_deps.contains(&d) {
                    visited_deps.insert(d.clone());
                    unique_deps.insert((dep.0.clone(), dep.1.clone()));
                    Some(d)
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        body
    }
}

#[cfg(test)]
mod tests {
    use std::vec;
    use crate::config::load_config;
    use super::*;

    #[test]
    fn test_collect_dependencies() {
        let config = load_config("src/cfg/cfg.xml");
        let depviz = Depviz::new(&config.binpath, &config.outputpath);

        let mut deps: Vec<(String, String)> = vec![];
        depviz.collect_dependencies("pip", &mut deps, 2);

        assert_eq!(deps.len(), 0);
    }

    #[test]
    fn test_form_uml_body() {
        let config = load_config("src/cfg/cfg.xml");
        let depviz = Depviz::new(&config.binpath, &config.outputpath);

        let dependencies: Vec<(String, String)> = vec![
            (String::from("package1"), String::from("dependency1")),
            (String::from("package2"), String::from("dependency2")),
            (String::from("package1"), String::from("dependency1")),
            (String::from("package2"), String::from("dependency3")),
            (String::from("package3"), String::from("dependency1")),
        ];

        assert_eq!(
            // hardcoded
            String::from("\"package1\" -> \"dependency1\"\n\"package2\" -> \"dependency2\"\n\"package2\" -> \"dependency3\"\n\"package3\" -> \"dependency1\""),
            depviz.form_uml_body(&dependencies)
        );
    }
}
