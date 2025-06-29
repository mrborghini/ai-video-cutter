mod components;
use std::io::Write;

use crate::components::FFmpeg;

const BAR_WIDTH: i32 = 16;

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
        bar.push('#')
    }

    // Add the amount of dashes to the string
    for _ in 0..num_dashes {
        bar.push('-')
    }

    // Add \r to not replace the progress bar
    print!("\r[{}] [{}/{}] {}%", bar, amount, total, percentage);

    // Remove anything in the way
    std::io::stdout().flush().unwrap();
}

fn main() {
    let ffmpeg = FFmpeg::new();
    println!("Detecting video information...");
    let video = ffmpeg.get_video_info("test-video.mkv".to_string());
    println!("Duration: {} seconds", video.time_seconds);
    println!("Frames: {}", video.frames);
    println!("Splitting video into parts...");
    let clips = ffmpeg.split_video_in_parts(video, on_progress);
}
