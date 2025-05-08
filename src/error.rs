use std::{path::PathBuf, process::ExitStatus};

use colored::Colorize;
use thiserror::Error;

#[macro_export]
macro_rules! bail_on_error {
    ($exit_code:expr) => {{
        std::process::exit($exit_code);
    }};
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum AppError {
    #[error("CLI> {0}")]
    Cli(#[from] CliErr),

    #[error("Processor> {0}")]
    Process(#[from] ProcessErr),

    #[error("Terminal> {0}")]
    Terminal(#[from] TerminalErr),
}

impl AppError {
    pub fn pretty(&self) -> String {
        format!("{}: {}", "[Error]".red(), self.to_string())
    }
}

impl AppError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Cli(e) => e.exit_code(),
            Self::Process(e) => e.exit_code(),
            Self::Terminal(e) => e.exit_code(),
        }
    }
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum CliErr {
    #[error("validation> {0}")]
    Validation(#[from] ArgsValidationErr),
}

impl CliErr {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Validation(_) => 2,
        }
    }
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ArgsValidationErr {
    #[error("Unsupported file extension: '{0}'. Supported types are: {1}")]
    InvalidExtension(String, String),

    #[error("Oops! The specified path doesn't exist: '{0}'")]
    PathDoesNotExist(PathBuf),

    #[error("Oops! The file extension does not exist: '{0}'")]
    ExtNotExist(PathBuf),

    #[error("Oops! The given path is not a file: '{0}'")]
    PathIsNotFile(PathBuf),

    #[error("Oops! It contains invalid character encoding, please change to UTF8 : {0}")]
    InvalidCharCode(PathBuf),
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ProcessErr {
    #[error("I/O> {0}")]
    Io(#[from] std::io::Error),

    #[error("Conveter> {0}")]
    Convert(#[from] ConvertErr),
}

impl ProcessErr {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Io(_) => 1,
            Self::Convert(_) => 1,
        }
    }
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum TerminalErr {
    #[error("Dimension> {0}")]
    Dimension(#[from] DimensionErr),
}

impl TerminalErr {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Dimension(_) => 3,
        }
    }
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum DimensionErr {
    #[error("Oops! Failed to get terminal size.")]
    SizeUnavailable(),
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ConvertErr {
    #[error("Oops! ffmpeg has terminated non-normally: {0}")]
    FfmpegUnexpectedExitStatus(ExitStatus),

    #[error("Oops! ffmpeg has terminated non-normally. {0}")]
    FfmpegCommandFailed(#[source] Box<dyn std::error::Error>), // Use Box<dyn std::error::Error>
}
