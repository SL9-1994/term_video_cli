use clap::Parser;
use std::path::PathBuf;

use crate::error::{ArgsValidationErr, CliErr};

const SUPPORTED_VIDEO_EXTS: [&str; 4] = ["mp4", "mkv", "avi", "webm"];
const SUPPORTED_IMAGE_EXTS: [&str; 4] = ["png", "jpg", "jpeg", "webp"];

#[derive(Parser)]
#[command(version, about = "This CLI can draw and play arbitrary videos and images to the terminal.", long_about = None)]
pub struct Cli {
    /// Enter the path of the video you wish to convert. (Supported extensions: mp4, mkv...)
    /// Note: Since the conversion is based on the terminal size at the time this option is executed, a terminal of a different size will not be drawn correctly.
    #[arg(short, long, value_name = "CONVERT_VIDEO_PATH")]
    pub file_path: Option<PathBuf>,

    /// Enter the path of the image you want to convert. (Supported extensions: png, jpeg...)
    /// Note: Terminals of different sizes will not render correctly as the conversion is based on the terminal size at the time this option is executed.
    #[arg(short, long, value_name = "CONVERT_IMAGE_PATH")]
    pub image_path: Option<PathBuf>,

    /// Play ascii_video with the converted image already prepared in tmp.
    #[arg(short, long)]
    pub play: bool,
}

impl Cli {
    pub fn new() -> Self {
        Cli::parse()
    }

    pub fn validate(&self) -> Result<(), CliErr> {
        self.validate_file_path()?;
        self.validate_image_path()?;

        Ok(())
    }

    fn validate_file_path(&self) -> Result<(), ArgsValidationErr> {
        let file_path = self.file_path.as_ref().unwrap(); // clapで存在が担保されるため，unwrapを使用

        if !file_path.exists() {
            Err(ArgsValidationErr::PathDoesNotExist(file_path.to_path_buf()))?;
        }
        if !file_path.is_file() {
            Err(ArgsValidationErr::PathIsNotFile(file_path.to_path_buf()))?;
        }

        let file_ext = file_path
            .extension()
            .ok_or_else(|| ArgsValidationErr::ExtNotExist(file_path.to_path_buf()))?;
        let file_ext_str = file_ext
            .to_str()
            .ok_or_else(|| ArgsValidationErr::InvalidCharCode(file_path.to_path_buf()))?;

        if !SUPPORTED_VIDEO_EXTS.contains(&file_ext_str) {
            Err(ArgsValidationErr::InvalidExtension(
                file_ext_str.to_string(),
                SUPPORTED_VIDEO_EXTS.join(", "),
            ))?
        }

        Ok(())
    }

    fn validate_image_path(&self) -> Result<(), ArgsValidationErr> {
        let file_path = self.file_path.as_ref().unwrap(); // clapで存在が担保されるため，unwrapを使用

        if !file_path.exists() {
            Err(ArgsValidationErr::PathDoesNotExist(file_path.to_path_buf()))?;
        }
        if !file_path.is_file() {
            Err(ArgsValidationErr::PathIsNotFile(file_path.to_path_buf()))?;
        }

        let file_ext = file_path
            .extension()
            .ok_or_else(|| ArgsValidationErr::ExtNotExist(file_path.to_path_buf()))?;
        let file_ext_str = file_ext
            .to_str()
            .ok_or_else(|| ArgsValidationErr::InvalidCharCode(file_path.to_path_buf()))?;

        if !SUPPORTED_IMAGE_EXTS.contains(&file_ext_str) {
            Err(ArgsValidationErr::InvalidExtension(
                file_ext_str.to_string(),
                SUPPORTED_IMAGE_EXTS.join(", "),
            ))?
        }

        Ok(())
    }
}
