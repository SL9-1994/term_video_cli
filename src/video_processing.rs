use std::fs;
use std::{path::PathBuf, process::Command};

pub struct VideoProcessor {
    video_path: PathBuf,
    output_dir: PathBuf,
}

impl VideoProcessor {
    /// Creates a new `VideoProcessor` instance.
    ///
    /// ## Arguments
    ///
    /// * `video_path` - The path to the video file.
    /// * `output_dir` - The path to the output directory.
    ///
    /// ## Returns
    ///
    /// A new `VideoProcessor` instance.
    ///
    /// ## Panics
    ///
    /// Panics if it fails to create the output directory.
    pub fn new(video_path: PathBuf, output_dir: PathBuf) -> Self {
        if !output_dir.exists() {
            fs::create_dir(&output_dir).expect("Failed to create output directory");
        } else {
            eprintln!("{} Directory already exists.", output_dir.display());
        }

        VideoProcessor {
            video_path,
            output_dir,
        }
    }

    /// Converts the video to a grayscale image at 24 fps.
    ///
    /// ## Arguments
    ///
    /// * `path` - The path to the video file. (Supported extensions: ffmpeg)
    ///
    /// ## Returns
    ///
    /// The path to the locally downloaded video.
    ///
    /// ## Notes
    ///
    /// * Running ffmpeg in a child process.
    /// * Since images are converted at 24 fps, the video may squeeze the capacity or cause unstable conversion and output.
    pub async fn convert_to_grayscale_and_frame(&self) {
        let output_format = self.output_dir.join("output_%03d.png");

        let status = Command::new("ffmpeg")
            .arg("-i")
            .arg(self.video_path.to_str().unwrap())
            .arg("-vf")
            .arg("fps=24, format=gray")
            .arg(output_format.to_str().unwrap())
            .status()
            .expect("Failed to execute command");

        assert!(status.success());
    }
}
