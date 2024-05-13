use clap::Parser;
use std::{env::current_dir, path::PathBuf};
use terminal::Terminal;
use video_processing::VideoProcessor;

mod downloder;
mod terminal;
mod video_processing;

use crate::downloder::yt_video_downloader;

/// Command line arguments for the CLI.
#[derive(Parser)]
#[command(version, about = "This is the terminal_ascii_video drawing CLI with video download capability.", long_about = None)]
struct Cli {
    /// Specify any YouTube URL to download the video.
    #[arg(short, long, value_name = "URL")]
    url: Option<String>,

    /// Enter the path of the video you wish to convert. (Supported extensions: mp4, mkv...)
    /// Note: Since the conversion is based on the terminal size at the time this option is executed, a terminal of a different size will not be drawn correctly.
    #[arg(short, long, value_name = "CONVERT_VIDEO_PATH")]
    file_path: Option<PathBuf>,

    /// Enter the path of the image you want to convert. (Supported extensions: png, jpeg...)
    /// Note: Terminals of different sizes will not render correctly as the conversion is based on the terminal size at the time this option is executed.
    #[arg(short, long, value_name = "CONVERT_IMAGE_PATH")]
    image_path: Option<PathBuf>,

    /// Play ascii_video with the converted image already prepared in tmp.
    #[arg(short, long)]
    play: bool,
}

/// Downloads a video from the given URL and returns the path to the downloaded video.
/// Arguments
/// * `url` - The URL of the video to download.
/// Returns
/// * The path to the downloaded video.
async fn download_video(url: &str) -> PathBuf {
    let local_video_path = yt_video_downloader(url.to_string()).await;
    let mut current_dir: PathBuf = current_dir().unwrap();
    current_dir.push(local_video_path);
    println!("The downloaded video path is {:?}", current_dir);
    current_dir
}

///  Performs the conversion process for the video of the given file path.
/// Arguments
/// * `file_path` - The path to the video file to process.
/// Returns
/// * `None`
async fn process_video(file_path: &PathBuf) {
    let terminal = Terminal::new();
    let tmp_dir = current_dir().unwrap().join("tmp");
    let processor = VideoProcessor::new(Some(file_path.to_path_buf()), Some(tmp_dir), &terminal);
    processor.convert_to_grayscale_and_frame().await;
}

/// Performs the conversion process for the image of the given file path.
/// Arguments
/// * `file_path` - The path to the image file to process.
/// Returns
/// * `None`
async fn process_image(file_path: &PathBuf) {
    let terminal = Terminal::new();
    let tmp_dir = current_dir().unwrap().join("tmp");
    let processor = VideoProcessor::new(Some(file_path.to_path_buf()), Some(tmp_dir), &terminal);
    processor.convert_to_grayscale_and_resize().await;
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Some(url) = cli.url.as_deref() {
        download_video(url).await;
    }

    if let Some(file_path) = cli.file_path.as_deref() {
        process_video(&file_path.to_path_buf()).await;
    }

    if let Some(image_path) = cli.image_path.as_deref() {
        process_image(&image_path.to_path_buf()).await;
    }

    if cli.play {
        let terminal = Terminal::new();
        let processor = VideoProcessor::new(None, None, &terminal);
        let frames = processor.convert_to_ascii_art().await;
        terminal.print_ascii_video(&frames).await;
    }
}
