#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate derive_new;

use std::process::Command;
use std::fs::{File, read_dir};
use std::io::Read;
use std::path::Path;
use std::ffi::OsStr;
use std::io::Write;

use serde_json::{Value};

// The path to where we copy the source in the docker container
const DOCKER_SRC_PATH: &'static str = "/code/";

#[derive(Serialize, PartialEq, Debug)]
struct Lines {
    begin: i64,
    end: i64,
}

#[derive(Serialize, PartialEq, Debug)]
struct Location {
    path: String,
    lines: Lines,
}

#[derive(new, Serialize, PartialEq, Debug)]
struct Issue {
    #[new(value = r#""issue""#)]
    #[serde(rename="type")]
    itype: &'static str,
    #[new(value = r#""rustfmt/style/rustfmt""#)]
    check_name: &'static str,
    #[new(value = r#""Your code does not pass rustfmt. Please format your code!""#)]
    description: &'static str,
    #[new(value = r#"vec!["Style"]"#)]
    categories: Vec<&'static str>,
    location: Location,
}

fn generate_location(file: &Path) -> Location {
    let path = file.to_str()
        .expect("Error getting path as str!")
        .to_owned()
        .replace(DOCKER_SRC_PATH, "");
    let lines = Lines { begin: 1, end: 1 };

    Location { path, lines }
}

fn generate_issue(file: &Path) -> Issue {
    let location = generate_location(file);

    Issue::new(location)
}

fn print_issue<'a>(issue: &'a Issue) {
    println!("{}\0",
             serde_json::to_string_pretty(&issue).expect("Error creating json from issue struct"))
}

fn get_include_paths() -> Vec<String> {
    if let Ok(mut config) = File::open("/config.json") {

        let mut content = String::new();
        config
            .read_to_string(&mut content)
            .expect("Error reading /config.json");

        let json: Value = serde_json::from_str(&content).expect("Error converting content to json");
        json["include_paths"]
            .as_array()
            .expect("Could not get include_paths")
            .iter()
            .map(|s| {
                     let mut path = DOCKER_SRC_PATH.to_string();
                     path.push_str(s.as_str().expect("Error extracting include path"));
                     path
                 })
            .collect::<Vec<String>>()
    } else {
        vec![DOCKER_SRC_PATH.to_string()]
    }
}

fn run_rustfmt_for_file(file: &Path) -> bool {
    Command::new("/app/rustfmt")
        .args(&["--write-mode=diff",
                &format!("--config-path={}", DOCKER_SRC_PATH),
                file.to_str().unwrap()])
        .output()
        .expect("Error executing rustfmt")
        .status
        .success()
}

fn process_file(file: &Path) {
    if !run_rustfmt_for_file(file) {
        let issue = generate_issue(file);
        print_issue(&issue);
    }
}

fn handle_path(path: &Path) {
    if path.is_file() {
        if path.extension() == Some(OsStr::new("rs")) {
            process_file(path);
        }
    } else {
        for new_path in read_dir(path).expect(&format!("Error reading directory: {:?}", path)) {
            handle_path(new_path.expect("Error getting DirEntry").path().as_path());
        }
    }
}

fn main() {
    let include_paths = get_include_paths();

    for path in include_paths.iter().map(Path::new) {
        handle_path(&path);
    }
}
