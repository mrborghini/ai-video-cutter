use std::path::Path;
use crate::components::run_command;

pub struct Video {
    pub path: String,
    pub time_seconds: f32,
    pub frames: i32,
    pub video_format: String,
}

pub struct FFmpeg {}

impl FFmpeg {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_video_info(&self, path: String) -> Video {
        let stderr = run_command("ffmpeg", &[
            "-i", &path, "-map", "0:v:0", "-c", "copy", "-f", "null", "-",
        ]);

        let mut frames: i32 = 0;
        let mut time_seconds: f32 = 0.0;

        for line in stderr.lines() {
            if line.contains("frame=") && line.contains("time=") {
                // Example line:
                // frame=65607 fps=... time=00:18:13.43 ...
                let frame_str = line
                    .split("frame=")
                    .nth(1)
                    .and_then(|s| s.split_whitespace().next())
                    .unwrap_or("0");

                let time_str = line
                    .split("time=")
                    .nth(1)
                    .and_then(|s| s.split_whitespace().next())
                    .unwrap_or("00:00:00.00");

                frames = frame_str.parse::<i32>().unwrap_or(0);
                time_seconds = Self::parse_time_to_seconds(time_str);
            }
        }

        let video_path = path.clone();
        let split_path: Vec<&str> = path.split(".").collect();
        let video_format = split_path[split_path.len() - 1];

        Video {
            path: video_path,
            time_seconds,
            frames,
            video_format: format!(".{}", video_format),
        }
    }

    pub fn extract_audio(&self, video_path: &str, output_path: &str) -> String {
        run_command("ffmpeg", &["-i", &video_path, &output_path]);
        output_path.to_string()
    }

    pub fn split_video_in_parts(
        &self,
        video: Video,
        out_dir: &str,
        on_progress: fn(amount: i32, total: i32),
    ) -> Vec<String> {
        let video_seconds = video.time_seconds as i32;

        let mut paths: Vec<String> = Vec::new();
        for i in 0..video_seconds {
            if i % 60 != 0 {
                continue;
            }
            let temp_path = Path::new(out_dir);
            let video_name = format!("{}{}", i, video.video_format);
            let out_path = temp_path.join(video_name).to_str().unwrap().to_string();
            on_progress(i, video_seconds);
            run_command("ffmpeg", &[
                "-i",
                &video.path,
                "-y",
                "-ss",
                &i.to_string(),
                "-t",
                "60",
                "-c",
                "copy",
                &out_path,
            ]);
            paths.push(out_path);
        }
        on_progress(video_seconds, video_seconds);
        paths
    }

    fn parse_time_to_seconds(time: &str) -> f32 {
        // time format: "HH:MM:SS.MS"
        let parts: Vec<&str> = time.split(':').collect();
        if parts.len() != 3 {
            return 0.0;
        }
        let hours = parts[0].parse::<f32>().unwrap_or(0.0);
        let minutes = parts[1].parse::<f32>().unwrap_or(0.0);
        let seconds = parts[2].parse::<f32>().unwrap_or(0.0);

        hours * 3600.0 + minutes * 60.0 + seconds
    }
}
