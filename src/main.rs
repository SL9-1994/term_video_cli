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
    #[arg(short, long, value_name = "url")]
    url: Option<String>,

    /// Enter the path of the video you wish to convert. (Supported extensions: mp4, mkv)
    #[arg(short, long, value_name = "FILE_PATH")]
    file_path: Option<PathBuf>,
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

/// Processes the video located at the given file path.
/// Arguments
/// * `file_path` - The path to the video file to process.
/// Returns
/// * None
async fn process_video(file_path: &PathBuf) {
    let terminal = Terminal::new();
    let tmp_dir = current_dir().unwrap().join("tmp");
    let processor = VideoProcessor::new(file_path.to_path_buf(), tmp_dir, &terminal);
    processor.convert_to_grayscale_and_frame().await;
    let frames = processor.convert_to_ascii_art().await;
    terminal.print_ascii_video(&frames).await;
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Some(url) = cli.url.as_deref() {
        let _video_path = download_video(url).await;
    }

    if let Some(file_path) = cli.file_path.as_deref() {
        process_video(&file_path.to_path_buf()).await;
    }
}
