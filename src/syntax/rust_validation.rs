
use godot::{builtin::{Dictionary, VariantArray}, log::{godot_error, godot_print}, meta::ToGodot};
use serde::Deserialize;
use std::{path::Path, process::Command};


#[derive(Debug, Deserialize)]
struct ClippyMessage {
    reason: String,
    target: Target,
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
    code: String
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
    let binding = file_name.to_string();
    let path = Path::new(&binding);
    let path = path.parent().map(|p| p.to_path_buf());
    let mut results = VariantArray::new();
    let output = Command::new("cargo")
        .current_dir(path)
        .args(["clippy", "--fix", "--allow-dirty", "--message-format=json"])
        .output()
        .expect("Failed to execute cargo clippy");

    // Check if the command was successful
    if !output.status.success() {
        godot_error!("cargo clippy failed");
    }

    // Parse the output
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if let Ok(message) = serde_json::from_str::<ClippyMessage>(line) {
            if message.reason == "compiler-message" && message.message.level == level {
                if let Some(span) = &message.message.spans {
                    for span in span {
                        if span.file_name == file_name {
                            let mut dictionary = Dictionary::new();
                            // int
                            dictionary.set("start_line".to_variant(), span.line_start.to_variant());
                            dictionary.set("end_line".to_variant(), span.line_end.to_variant());
                            dictionary.set("leftmost_column".to_variant(), span.column_start.to_variant());
                            dictionary.set("rightmost_column".to_variant(), span.column_end.to_variant());
                            dictionary.set("code".to_variant(), 1.to_variant());
                            // string
                            if let Some(code) = &message.message.code {
                                dictionary.set("string_code".to_variant(), code.code.to_variant());
                            } else {
                                dictionary.set("string_code".to_variant(), "".to_variant());
                            }
                            dictionary.set("message".to_variant(), message.message.message.to_variant());
        
                            results.push(dictionary.to_variant());
                        }
                    }
                }
            } else {
                // what is reason?
                
                godot_print!("Failed to parse clippy message: {}", line);
            }
        }
    }
    results
}
fn print_clippy_message(message: &Message) {
    godot_print!("{}: {}", message.level, message.message);
}