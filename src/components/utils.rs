use std::process::Command;

pub fn run_command(program: &str, args: &[&str]) -> String {
        let output = Command::new(program)
            .args(args)
            .output()
            .expect("Failed to run ffmpeg");

        let output_str = String::from_utf8_lossy(&output.stderr).to_string();
        output_str
    }