use clap::Parser;
use std::{env::current_dir, path::PathBuf};
use terminal::Terminal;
use video_processor::VideoProcessor;

mod terminal;
mod video_processor;

#[derive(Parser)]
#[command(version, about = "This CLI can draw and play arbitrary videos and images to the terminal.", long_about = None)]
struct Cli {
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

///  Performs the conversion process for the video of the given file path.
/// Arguments
/// * `file_path` - The path to the video file to process.
/// Returns
/// * `None`
fn process_video(file_path: &PathBuf) {
    let terminal = Terminal::new();
    let tmp_dir = current_dir().unwrap().join("tmp");
    let processor = VideoProcessor::new(Some(file_path.to_path_buf()), Some(tmp_dir), &terminal);
    processor.convert_to_grayscale_and_frame();
}

/// Performs the conversion process for the image of the given file path.
/// Arguments
/// * `file_path` - The path to the image file to process.
/// Returns
/// * `None`
fn process_image(file_path: &PathBuf) {
    let terminal = Terminal::new();
    let tmp_dir = current_dir().unwrap().join("tmp");
    let v_processor = VideoProcessor::new(Some(file_path.to_path_buf()), Some(tmp_dir), &terminal);
    v_processor.convert_to_grayscale_and_resize();
}

fn main() {
    let cli = Cli::parse();

    if let Some(file_path) = cli.file_path {
        process_video(&file_path.to_path_buf());
    }

    if let Some(image_path) = cli.image_path {
        process_image(&image_path.to_path_buf());
    }

    if cli.play {
        let terminal = Terminal::new();
        let processor = VideoProcessor::new(None, None, &terminal);
        let frames = processor.convert_to_ascii_art();
        terminal.print_ascii_video(&frames);
    }
}
