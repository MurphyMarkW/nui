use std::process::Command;
use std::str;

static NMAPBIN: &str = "nmap";

#[derive(Debug)]
pub struct Nmap<'a> {
    pub name: &'a str,
    pub hosts: Vec<String>,
}

impl Nmap<'_> {
    pub fn version(&self) -> String {
        let output = Command::new(NMAPBIN)
                             .arg("--version")
                             .output()
                             .expect("failed to execute process");

        str::from_utf8(&output.stdout).unwrap().to_owned()
    }

    pub fn help(&self) -> String {
        let output = Command::new(NMAPBIN)
                             .arg("--help")
                             .output()
                             .expect("failed to execute process");

        str::from_utf8(&output.stdout).unwrap().to_owned()
    }

    pub fn scan(&self) -> String {
        let output = Command::new(NMAPBIN)
                             .arg(&self.hosts[0])
                             .arg("-oX")
                             .arg("-")
                             .output()
                             .expect("failed to execute process");

        str::from_utf8(&output.stdout).unwrap().to_owned()
    }
}
