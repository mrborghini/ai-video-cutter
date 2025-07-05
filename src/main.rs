mod components;
use rust_logger::{Logger, Severity};
use std::{fs, io::Write};

use crate::components::{FFmpeg, LLM, Ollama, Whisper, WhisperJsonOutput};

const BAR_WIDTH: i32 = 16;
const OUT_DIR: &str = "output";

fn select_llm() -> impl LLM {
    Ollama::new()
}

fn on_progress(amount: i32, total: i32) {
    let mut bar: String = String::new();

    // Calculate the percentage
    let percentage: i32 = (amount * 100) / total;

    // Calculate number of hashtags by using the width
    let num_hashtags = (percentage * BAR_WIDTH) / 100;

    // Calculate the number of dashes
    let num_dashes = BAR_WIDTH - num_hashtags;

    // Add the amount of hashtags to the string
    for _ in 0..num_hashtags {
        bar.push('█')
    }

    // Add the amount of dashes to the string
    for _ in 0..num_dashes {
        bar.push('▒')
    }

    // Add \r to not replace the progress bar
    print!("\r[{}] [{}/{}] {}%", bar, amount, total, percentage);

    // Remove anything in the way
    std::io::stdout().flush().unwrap();
}

fn main() {
    let log = Logger::new("main");
    let llm = select_llm();
    let ffmpeg = FFmpeg::new();
    let whisper = Whisper::new(
        "./whisper.cpp/models/ggml-large-v3-turbo.bin".to_string(),
        "./whisper.cpp/build/bin/whisper-cli".to_string(),
    );

    log.info("Step 1: Detecting video information...");
    let video = ffmpeg.get_video_info("test-video.mkv".to_string());
    log.debug(format!("Duration: {} seconds", video.time_seconds));
    log.debug(format!("Frames: {}", video.frames));

    if video.time_seconds == 0.0 {
        log.error("Video is 0 seconds long", Severity::Critical);
        return;
    }

    if video.frames == 0 {
        log.error("Video doesn't have any frames", Severity::Critical);
        return;
    }

    log.info("Step 2: Splitting video into parts...");
    let _ = fs::create_dir(OUT_DIR);
    let clips = ffmpeg.split_video_in_parts(video, OUT_DIR, on_progress);
    println!();

    log.info("Step 3: Transcribing audio from clips...");
    let total_clips = clips.len();
    let mut clip_transcriptions: Vec<WhisperJsonOutput> = Vec::new();
    for (i, clip) in clips.iter().enumerate() {
        on_progress(i as i32 + 1, total_clips as i32);
        let transcription = whisper.analyze_audio(clip.to_string(), OUT_DIR);
        clip_transcriptions.push(transcription);
    }

    for (i, transcription) in clip_transcriptions.iter().enumerate() {
        let segment_start = i as f32 * 60.0;
        let segment_end = segment_start + 60.0;

        println!("--- Segment {}s - {}s ---", segment_start, segment_end);

        for transcription_result in &transcription.transcription {
            println!(
                "{}s - {}s: {}",
                transcription_result.offsets.from as f32 / 1000.0,
                transcription_result.offsets.to as f32 / 1000.0,
                transcription_result.text
            );
        }
    }
}
