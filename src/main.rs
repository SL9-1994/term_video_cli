use cli::Cli;
use error::AppError;
use std::{env::current_dir, path::PathBuf};
use terminal::Terminal;
use video_processor::VideoProcessor;

mod cli;
mod error;
mod terminal;
mod video_processor;

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error.pretty());

        let exit_code = error.exit_code();

        bail_on_error!(exit_code);
    }
}

fn run() -> Result<(), AppError> {
    let cli = Cli::new();
    cli.validate()?;

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

    Ok(())
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
