pub struct Terminal {
    // terminal_size
    width: Option<usize>,
    height: Option<usize>,
}

impl Terminal {
    /// Creates a new instance of the Terminal struct.
    ///
    /// This function initializes the Terminal struct with the current terminal dimensions.
    /// If the dimensions cannot be determined, it sets the width and height to 0.
    ///
    /// ### Returns
    /// The newly created Terminal instance.
    pub fn new() -> Self {
        let (width, height) = term_size::dimensions().unwrap_or((0, 0));
        Terminal {
            width: Some(width),
            height: Some(height),
        }
    }

    /// Prints the terminal size in ASCII art.
    ///
    /// This function prints the current terminal size using ASCII characters.
    /// Each ASCII character represents a different brightness level.
    ///
    /// ## Arguments
    /// * None
    ///
    /// ## Returns
    /// * None
    pub async fn print_term_ascii(&self) {
        // ASCII characters corresponding to different brightness levels
        // let ascii_brightness = ['@', '#', 'S', '%', '?', '*', '+', ';', ':', ',', '.'];
    }
}
