use std::process::Command;
use std::str;

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

    fn run_ffmpeg(&self, args: &[&str]) -> String {
        let output = Command::new("ffmpeg")
            .args(args)
            .output()
            .expect("Failed to run ffmpeg");

        let output_str = String::from_utf8_lossy(&output.stderr).to_string();
        output_str
    }

    pub fn get_video_info(&self, path: String) -> Video {
        let stderr = self.run_ffmpeg(&[
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

    pub fn split_video_in_parts(&self, video: Video) -> Vec<String> {
        let video_seconds = video.time_seconds as i32;
        let mut paths: Vec<String> = Vec::new();
        for i in 0..video_seconds {
            if i % 60 != 0 {
                continue;
            }
            let path = format!("{}{}", i, video.video_format);
            self.run_ffmpeg(&[
                "-i",
                &video.path,
                "-y",
                "-ss",
                &i.to_string(),
                "-t",
                "60",
                "-c",
                "copy",
                &path,
            ]);
            paths.push(path);
        }
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
