use crate::components::FFmpeg;

pub struct Whisper {
    ffmpeg: FFmpeg,
    model_path: String,
}

impl Whisper {
    pub fn new(model_path: String) -> Self {
        Self {
            ffmpeg: FFmpeg::new(),
            model_path,
        }
    }

    pub fn analyze_audio(&self, video: String) {
        let output = self.ffmpeg.extract_audio(&video);
        println!("{}", output);
    }
}
