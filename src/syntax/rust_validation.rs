use std::env;
use std::path::Path;
use std::process::Command;

use godot::builtin::Dictionary;
use godot::builtin::VariantArray;
use godot::global::godot_error;
use godot::global::godot_print;
use godot::meta::ToGodot;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct ClippyMessage {
    reason: String,
    target: Target,
    manifest_path: String,
    message: Message,
}
#[derive(Debug, Deserialize)]
struct Target {
    src_path: String,
}
#[derive(Debug, Deserialize)]
struct Message {
    code: Option<Code>,
    level: String,
    message: String,
    spans: Option<Vec<Span>>,
}
#[derive(Debug, Deserialize)]
struct Code {
    code: String,
}
#[derive(Debug, Deserialize)]
struct Span {
    file_name: String,
    line_end: i32,
    line_start: i32,
    column_end: i32,
    column_start: i32,
}
pub fn run_checks(file_name: String, level: String) -> VariantArray {
    // remove res:// from filename
    let file_name = file_name.replace("res://", "");
    let binding = file_name.to_string();
    let path = Path::new(&binding);
    let path = path.parent();
    let mut current_folder = env::current_dir().unwrap();
    let mut results = VariantArray::new();
    // create current dir folder
    let mut current_dir = Path::new(".");
    if let Some(path) = &path {
        current_dir = path;
    }
    current_folder.push(current_dir);
    godot_print!("{}", current_folder.display());
    let output = Command::new("cargo")
        .args(["clippy", "--message-format=json"])
        .current_dir(&current_folder)
        .output();
    if output.is_err() {
        godot_error!(
            "cargo clippy failed at {}:\n {}",
            current_folder.display(),
            output.unwrap_err()
        );
        return results;
    }
    let output = output.unwrap();
    // Parse the output
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if let Ok(message) = serde_json::from_str::<ClippyMessage>(line) {
            if message.reason == "compiler-message" && message.message.level == level {
                // get parent of manifest path
                let manifest_path = Path::new(&message.manifest_path);
                let manifest_path = manifest_path.parent();
                if manifest_path.is_none() {
                    continue;
                }
                let manifest_path = manifest_path.unwrap();
                let manifest_folder = manifest_path.file_name();
                if manifest_folder.is_none() {
                    continue;
                }
                if let Some(span) = &message.message.spans {
                    for span in span {
                        let mut manifest_folder =
                            manifest_folder.unwrap().to_str().unwrap().to_string();
                        manifest_folder.push('/');
                        manifest_folder.push_str(&span.file_name);
                        let span_filename = manifest_folder;
                        if span_filename == file_name {
                            let mut dictionary = Dictionary::new();
                            if level == "error" {
                                // int
                                dictionary.set("line".to_variant(), span.line_start.to_variant());
                                dictionary
                                    .set("column".to_variant(), span.column_start.to_variant());
                                // string
                                dictionary.set(
                                    "message".to_variant(),
                                    message.message.message.to_variant(),
                                );
                            }
                            if level == "warning" {
                                // int
                                dictionary
                                    .set("start_line".to_variant(), span.line_start.to_variant());
                                dictionary.set("end_line".to_variant(), span.line_end.to_variant());
                                dictionary.set(
                                    "leftmost_column".to_variant(),
                                    span.column_start.to_variant(),
                                );
                                dictionary.set(
                                    "rightmost_column".to_variant(),
                                    span.column_end.to_variant(),
                                );
                                dictionary.set("code".to_variant(), 1.to_variant());
                                // string
                                if let Some(code) = &message.message.code {
                                    dictionary
                                        .set("string_code".to_variant(), code.code.to_variant());
                                } else {
                                    dictionary.set("string_code".to_variant(), "".to_variant());
                                }
                                dictionary.set(
                                    "message".to_variant(),
                                    message.message.message.to_variant(),
                                );
                            }
                            results.push(dictionary.to_variant());
                        }
                    }
                }
            } else {
                // what is reason?

                //godot_print!("Failed to parse clippy message: {}", line);
            }
        }
    }
    results
}
fn print_clippy_message(message: &Message) {
    godot_print!("{}: {}", message.level, message.message);
}
