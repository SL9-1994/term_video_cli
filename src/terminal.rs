use std::{thread::sleep, time::Duration};

use crate::error::{DimensionErr, TerminalErr};

#[derive(Clone)]
pub struct Terminal {
    // terminal_size
    pub width: Option<usize>,
    pub height: Option<usize>,
}

impl Terminal {
    /// Creates a new instance of the Terminal struct.
    ///
    /// This function initializes a new instance of the Terminal struct with the current terminal dimensions.
    ///
    /// ## Returns
    /// * A new instance of the Terminal struct.
    pub fn new() -> Result<Self, TerminalErr> {
        let (width, height) = term_size::dimensions().ok_or(DimensionErr::SizeUnavailable())?;
        Ok(Terminal {
            width: Some(width),
            height: Some(height),
        })
    }

    /// Moves the cursor to the top-left corner of the terminal.
    ///
    /// This function moves the cursor to the top-left corner of the terminal by printing the appropriate escape sequence.
    ///
    /// ## Returns
    /// * None
    pub fn move_cursor_to_top(&self) {
        print!("\x1B[1;1H");
    }

    /// Prints an ASCII video to the terminal.
    ///
    /// This function prints an ASCII video to the terminal by displaying each frame in the provided frames array.
    /// It moves the cursor to the top of the terminal before printing each frame, creating the effect of an animated video.
    /// The frame rate is controlled by sleeping for a specific duration between each frame.
    ///
    /// ## Arguments
    /// * `frames` - A slice of vectors containing strings representing each frame of the video.
    ///
    /// ## Returns
    /// * None
    pub fn print_ascii_video(&self, frames: &[Vec<String>]) {
        for frame in frames {
            self.move_cursor_to_top();
            for line in frame {
                print!("{}", line);
            }
            // Sleep for a while to control the frame rate
            sleep(Duration::from_millis(1000 / 24));
        }
    }
}
