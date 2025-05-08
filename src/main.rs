use cli::Cli;
use error::{AppError, ProcessErr};
use std::path::PathBuf;
use terminal::Terminal;
use util::get_tmp_dir;
use video_processor::VideoProcessor;

mod cli;
mod error;
mod terminal;
mod util;
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
    // -pオプションオンリーの場合は関係のないバリデーションを行わない
    if !cli.play && cli.file_path.is_none() || !cli.image_path.is_none() {
        cli.validate()?;
    }

    let terminal = Terminal::new()?;
    let tmp_path = get_tmp_dir()?;

    if let Some(file_path) = cli.file_path {
        process_video(&file_path.to_path_buf(), tmp_path.clone(), terminal.clone())?;
    }

    if let Some(image_path) = cli.image_path {
        process_image(
            &image_path.to_path_buf(),
            tmp_path.clone(),
            terminal.clone(),
        )?;
    }

    if cli.play {
        let terminal = Terminal::new()?;
        let processor = VideoProcessor::new(None, None, &terminal)?;
        let frames = processor.convert_to_ascii_art()?;
        terminal.print_ascii_video(&frames);
    }

    Ok(())
}

///  Performs the conversion process for the video of the given file path.
/// Arguments
/// * `file_path` - The path to the video file to process.
/// Returns
/// * `None`
fn process_video(
    file_path: &PathBuf,
    tmp_path: PathBuf,
    terminal: Terminal,
) -> Result<(), ProcessErr> {
    let processor = VideoProcessor::new(Some(file_path.to_path_buf()), Some(tmp_path), &terminal)?;
    processor.convert_to_grayscale_and_frame()?;
    Ok(())
}

/// Performs the conversion process for the image of the given file path.
/// Arguments
/// * `file_path` - The path to the image file to process.
/// Returns
/// * `None`
fn process_image(
    file_path: &PathBuf,
    tmp_path: PathBuf,
    terminal: Terminal,
) -> Result<(), ProcessErr> {
    let v_processor =
        VideoProcessor::new(Some(file_path.to_path_buf()), Some(tmp_path), &terminal)?;
    v_processor.convert_to_grayscale_and_resize()?;
    Ok(())
}
