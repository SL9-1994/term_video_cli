use clap::Parser;
use std::{env::current_dir, path::PathBuf};
use terminal::Terminal;
use video_processing::VideoProcessor;
mod downloder;
mod terminal;
mod video_processing;
use crate::downloder::yt_video_downloader;
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

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let mut current_dir: PathBuf = current_dir().unwrap();
    if let Some(url) = cli.url.as_deref() {
        let local_video_path = yt_video_downloader(url.to_string()).await;
        current_dir.push(local_video_path);

        println!("The downloaded video path is {:?}", current_dir);
    }

    if let Some(file_path) = cli.file_path.as_deref() {
        let tmp_dir = current_dir.join("tmp");
        let processor = VideoProcessor::new(file_path.to_path_buf(), tmp_dir);
        processor.convert_to_grayscale_and_frame().await;

        let terminal = Terminal::new();
        // ターミナルへasciiを出力する。
        terminal.print_term_ascii().await;
    }
}
