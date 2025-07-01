use std::fs;
use std::path::{Path, PathBuf};

use crate::components::{FFmpeg, run_command};

pub struct Whisper {
    ffmpeg: FFmpeg,
    model_path: String,
    whisper_cli_path: String,
}

impl Whisper {
    pub fn new(model_path: String, whisper_cli_path: String) -> Self {
        Self {
            ffmpeg: FFmpeg::new(),
            model_path,
            whisper_cli_path,
        }
    }

    pub fn analyze_audio(&self, video: String, out_dir: &str) -> String {
        let video_path = Path::new(&video);
        let video_name = video_path.file_name().unwrap().to_str().unwrap();
        let output_path = Path::new(out_dir);
        let _ = fs::create_dir(&output_path);
        let audio_path = output_path.join(format!("{}.wav", &video_name));
        let output = self
            .ffmpeg
            .extract_audio(&video, audio_path.to_str().unwrap());

        let command_output = run_command(
            &self.whisper_cli_path,
            &["-m", &self.model_path, &output, "--output-json"],
        );
        println!("{}", command_output);
        let _ = fs::remove_file(audio_path);
        "".to_string()
    }
}
