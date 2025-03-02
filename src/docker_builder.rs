use crate::common::print_message;
use crate::finder::find;
use std::fs::{self, ReadDir};
use std::io;
use std::path::PathBuf;
use std::time::Duration;

const DOCKER_FILE: &str = "Dockerfile";
const DELAY: Duration = Duration::from_millis(7);
// load current diretorory from filesystem
pub fn get_directory() -> ReadDir {
    let dir = ".";
    fs::read_dir(dir).unwrap_or_else(|e| panic!("{}", e))
}

fn user_input(options: usize) -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let parsed = input.trim().parse::<i32>();
    let r = match parsed {
        Ok(v) => v - 1,
        Err(_) => {
            print_message("Choose right option ", DELAY);
            user_input(options)
        }
    };
    let result = match r > options as i32 || r < 0 {
        true => {
            print_message("Choose right option ", DELAY);
            user_input(options)
        }
        _ => r,
    };
    result
}

pub fn process() {
    let dir = get_directory();
    let docker_files = find(dir, DOCKER_FILE, Vec::new());

    match docker_files.len() {
        0 => NotFoundDockerAction {}.action(),
        1 => SimpleDockerAction {
            files: docker_files,
        }
        .action(),
        _ => MultiFilesDockerAction {
            files: docker_files,
        }
        .action(),
    }
}

/// Load a file from the given path as a string.
///
/// # Parameters
/// - `path`: A reference to a `PathBuf` representing the path of the file to be read.
///
/// # Returns
/// - A `String` containing the content of the file. If an error occurs during reading, it will panic with the error message.
/// ```
fn load(path: &PathBuf) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("{}", e))
}

/// Parse the input content into a vector of strings, splitting by newline characters.
/// It will filter out any lines that don't start with "ENV" and trim any whitespace from each line.
///
/// # Parameters
/// * `content`: The input string to parse.
///
/// # Return type
/// A vector of strings, where each string is a trimmed line from the original content that starts with "ENV".
fn parse(content: String) -> Vec<String> {
    content
        .split('\n')
        .map(|line| line.trim().to_string())
        .filter(|str| str.starts_with("ENV"))
        .collect()
}

fn build_command(vec: Vec<String>) -> Vec<String> {
    vec.iter()
        .map(|s| {
            s.replace("$", "")
                .replacen("ENV", "", 1)
                .to_string()
                .replace(" ", "=")
                .replacen("=", " ", 1)
        })
        .map(|s| format!("--build-arg{}", s))
        .collect::<Vec<_>>()
}

trait Action {
    fn action(&self);
}
impl Action for SimpleDockerAction {
    fn action(&self) {
        let head = self.files.iter().next().unwrap();
        let content = load(head);
        let parsed = parse(content);
        let command = build_command(parsed).join(" ");
        print_message(&command, Duration::from_millis(1));
    }
}
impl Action for NotFoundDockerAction {
    fn action(&self) {
        print_message("Dockerfile not found", DELAY);
    }
}
impl Action for MultiFilesDockerAction {
    fn action(&self) {
        let list_files_str = self
            .files
            .iter()
            .enumerate()
            .map(|(index, p)| format!("{}: {} \n", index + 1, p.as_path().to_str().unwrap()))
            .collect::<Vec<String>>()
            .join("");
        let message = format!(
            "Found list of Dockerfile... Choose one of them...\n{}",
            list_files_str
        );
        print_message(&message, DELAY);
        let index = user_input(self.files.len() - 1);
        let docker_file = self.files.get((index) as usize).unwrap();
        SimpleDockerAction {
            files: vec![docker_file.clone()],
        }
        .action();
    }
}

struct SimpleDockerAction {
    files: Vec<PathBuf>,
}

struct NotFoundDockerAction {}
struct MultiFilesDockerAction {
    files: Vec<PathBuf>,
}
