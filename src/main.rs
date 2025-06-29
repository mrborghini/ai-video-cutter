mod components;
use crate::components::FFmpeg;

fn main() {
    let ffmpeg = FFmpeg::new();
    let video = ffmpeg.get_video_info("test-video.mkv".to_string());
    println!("Path: {}", video.path);
    println!("Duration (s): {}", video.time_seconds);
    println!("Frames: {}", video.frames);
    let clips = ffmpeg.split_video_in_parts(video);
    println!("{:?}", clips);
}
