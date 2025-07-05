use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use rust_logger::Logger;
use serde::Deserialize;

use crate::components::{FFmpeg, run_command};

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct WhisperModelAudioOrTextObject {
    pub ctx: i32,
    pub state: i32,
    pub head: i32,
    pub layer: i32,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct WhisperModelObject {
    #[serde(rename = "type")]
    pub model_type: String,
    pub multilingual: bool,
    pub vocab: i32,
    pub audio: WhisperModelAudioOrTextObject,
    pub text: WhisperModelAudioOrTextObject,
    pub mels: i32,
    pub ftype: i32,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct WhisperParamsObject {
    pub model: String,
    pub language: String,
    pub translate: bool,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct WhisperResultObject {
    pub language: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct WhisperTranscriptionTimestampObject {
    pub from: String,
    pub to: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct WhisperTranscriptionOffsetsObject {
    pub from: i32,
    pub to: i32,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct WhisperTranscriptionObject {
    pub timestamps: WhisperTranscriptionTimestampObject,
    pub offsets: WhisperTranscriptionOffsetsObject,
    pub text: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct WhisperJsonOutput {
    #[serde(rename = "systeminfo")]
    pub system_info: String,
    pub model: WhisperModelObject,
    pub params: WhisperParamsObject,
    pub result: WhisperResultObject,
    pub transcription: Vec<WhisperTranscriptionObject>,
}

pub struct Whisper {
    ffmpeg: FFmpeg,
    model_path: String,
    whisper_cli_path: String,
    log: Logger,
}

impl Whisper {
    pub fn new(model_path: String, whisper_cli_path: String) -> Self {
        Self {
            ffmpeg: FFmpeg::new(),
            model_path,
            whisper_cli_path,
            log: Logger::new("Whisper"),
        }
    }

    pub fn analyze_audio(&self, video: String, out_dir: &str) -> WhisperJsonOutput {
        let video_path = Path::new(&video);
        let video_name = video_path.file_name().unwrap().to_str().unwrap();
        let output_path = Path::new(out_dir);
        let _ = fs::create_dir(&output_path);
        let audio_path = output_path.join(format!("{}.wav", &video_name));
        self.log.debug(audio_path.to_str().unwrap());
        let output = self
            .ffmpeg
            .extract_audio(&video, audio_path.to_str().unwrap());

        run_command(
            &self.whisper_cli_path,
            &["-m", &self.model_path, &output, "--output-json"],
        );
        let _ = fs::remove_file(&audio_path);
        let output_file_path = format!("{}.json", audio_path.to_str().unwrap());
        let mut json_file = File::open(output_file_path).expect("Failed to read json file");
        let mut buf = String::new();
        json_file
            .read_to_string(&mut buf)
            .expect("Failed to read content of json file");
        let parsed_output =
            serde_json::from_str::<WhisperJsonOutput>(&buf).expect("Could not parse json file");
        parsed_output
    }
}
