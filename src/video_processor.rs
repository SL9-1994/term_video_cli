use image::{DynamicImage, GenericImageView, Pixel};
use std::fs;
use std::path::Path;
use std::{path::PathBuf, process::Command};

use crate::error::{ConvertErr, ProcessErr};
use crate::terminal::Terminal;

pub struct VideoProcessor {
    terminal: Terminal,
    video_path: Option<PathBuf>,
    output_dir: Option<PathBuf>,
}

impl VideoProcessor {
    /// Creates a new `VideoProcessor` instance.
    ///
    /// ## Arguments
    ///
    /// * `video_path` - The path to the video file.
    /// * `output_dir` - The path to the output directory.
    /// * `terminal` - The reference to the terminal instance.
    ///
    /// ## Returns
    ///
    /// A new `VideoProcessor` instance.
    ///
    /// ## Panics
    ///
    /// Panics if it fails to create the output directory.
    pub fn new(
        video_path: Option<PathBuf>,
        output_dir: Option<PathBuf>,
        terminal: &Terminal,
    ) -> Result<Self, ProcessErr> {
        if let Some(dir) = &output_dir {
            if !dir.exists() {
                fs::create_dir(dir)?;
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    format!("{} Directory already exists.", dir.display()),
                ))?;
            }
        }

        Ok(VideoProcessor {
            video_path,
            output_dir,
            terminal: terminal.clone(),
        })
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
    pub fn convert_to_grayscale_and_frame(&self) -> Result<(), ProcessErr> {
        let output_format = match &self.output_dir {
            Some(dir) => dir.join("output_%04d.png"),
            None => PathBuf::from("output_%04d.png"),
        };

        // fps24で画像生成し、グレースケールに変換した後、ターミナルサイズに合わせて画像をリサイズする。
        let status = Command::new("ffmpeg")
            .arg("-i")
            .arg(
                self.video_path
                    .as_ref()
                    .unwrap()
                    .to_str()
                    .expect("failed to get video path"),
            )
            .arg("-vf")
            .arg(format!(
                "format=gray, scale={}:{}",
                self.terminal.width.expect("failed to get terminal width"),
                self.terminal.height.expect("failed to get terminal height")
            ))
            .arg(output_format.to_str().expect("failed to get output format"))
            .status();

        match status {
            Ok(status) if status.success() => Ok(()), // 成功の場合
            Ok(status) => Err(ConvertErr::FfmpegUnexpectedExitStatus(status))?, // 非正常終了
            Err(e) => Err(ConvertErr::FfmpegCommandFailed(Box::new(e)))?, // コマンド自体の実行失敗
        }
    }

    pub fn convert_to_grayscale_and_resize(&self) -> Result<(), ProcessErr> {
        let output_format = match &self.output_dir {
            Some(dir) => dir.join("single_output.png"),
            None => PathBuf::from("single_output.png"),
        };

        // fps24で画像生成し、グレースケールに変換した後、ターミナルサイズに合わせて画像をリサイズする。
        let status = Command::new("ffmpeg")
            .arg("-i")
            .arg(
                self.video_path
                    .as_ref()
                    .unwrap()
                    .to_str()
                    .expect("failed to get video path"),
            )
            .arg("-vf")
            .arg(format!(
                "format=gray, scale={}:{}",
                self.terminal.width.expect("failed to get terminal width"),
                self.terminal.height.expect("failed to get terminal height")
            ))
            .arg(output_format.to_str().expect("failed to get output format"))
            .status();

        match status {
            Ok(status) if status.success() => Ok(()), // 成功の場合
            Ok(status) => Err(ConvertErr::FfmpegUnexpectedExitStatus(status))?, // 非正常終了
            Err(e) => Err(ConvertErr::FfmpegCommandFailed(Box::new(e)))?, // コマンド自体の実行失敗
        }
    }

    /// Converts the image to ASCII art representation.
    ///
    /// ## Arguments
    ///
    /// * `img` - The image to convert.
    ///
    /// ## Returns
    ///
    /// The ASCII art representation of the image as a vector of strings.
    pub fn convert_image_to_ascii_art(&self, img: &DynamicImage) -> Vec<String> {
        let ascii_brightness = ['@', '#', 'S', '%', '?', '*', '+', ';', ':', ',', '.'];

        let (width, height) = img.dimensions();
        let mut ascii_art = Vec::new();

        for y in 0..height {
            let mut line = String::new();
            for x in 0..width {
                let pixel = img.get_pixel(x, y);
                let brightness = pixel.to_luma()[0];
                let ascii_char = Self::map_brightness_to_ascii(brightness, &ascii_brightness);
                line.push(ascii_char);
            }
            ascii_art.push(line);
        }

        ascii_art
    }

    /// Converts the video frames to ASCII art representation.
    ///
    /// ## Returns
    ///
    /// The ASCII art representation of the video frames as a vector of vectors of strings.
    pub fn convert_to_ascii_art(&self) -> Result<Vec<Vec<String>>, ProcessErr> {
        let mut entries: Vec<_> = fs::read_dir("./tmp")?.collect();
        entries.sort_by_key(|e| e.as_ref().unwrap().path().display().to_string());
        let mut frames = Vec::new();

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().unwrap_or_default() == "png" {
                let img = image::open(&Path::new(&path)).unwrap();
                let ascii_art = self.convert_image_to_ascii_art(&img);
                frames.push(ascii_art);
            }
        }

        Ok(frames)
    }

    /// Maps the brightness value to an ASCII character.
    ///
    /// ## Arguments
    ///
    /// * `brightness` - The brightness value.
    /// * `ascii_brightness` - The array of ASCII characters representing different brightness levels.
    ///
    /// ## Returns
    ///
    /// The ASCII character corresponding to the brightness value.
    pub fn map_brightness_to_ascii(brightness: u8, ascii_brightness: &[char]) -> char {
        let scale: usize = ascii_brightness.len() - 1;
        let index = (brightness as f32 / 255.0 * scale as f32).round() as usize;
        ascii_brightness[index]
    }
}
