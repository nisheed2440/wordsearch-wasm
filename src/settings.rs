use crate::orientations::*;

pub struct Settings {
    // The recommended width of the puzzle
    ///
    // **Note:** This will automatically increment if
    // the words cannot be placed properly in the puzzle
    pub width: i32,
    // The recommended height of the puzzle
    ///
    // **Note:** This will automatically increment if
    // the words cannot be placed properly in the puzzle
    pub height: i32,
    // The allowed orientations for the words placed in the puzzle
    pub orientations: Vec<Orientation>,
    // The allowed orientations for the words placed in the puzzle
    pub max_attempts: i32,
    // Maximum numbed of times the grid can grow
    // depending on the length of the words and placement
    pub max_grid_growth: i32,
    // Allow overlaping of words in the puzzle
    pub prefer_overlap: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            width: 10,
            height: 10,
            orientations: vec![
                Orientation::Horizontal,
                Orientation::Vertical,
                Orientation::Diagonal,
                Orientation::DiagonalUp,
            ],
            max_attempts: 10,
            max_grid_growth: 10,
            prefer_overlap: false,
        }
    }
}

impl Settings {
    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings() {
        let mut settings = Settings::default();
        assert_eq!(settings.width, 10);
        assert_eq!(settings.height, 10);
        assert_eq!(settings.max_attempts, 10);
        assert_eq!(settings.max_grid_growth, 10);
        assert_eq!(settings.prefer_overlap, false);

        settings.set_width(20);
        settings.set_height(20);
        assert_eq!(settings.width, 20);
        assert_eq!(settings.height, 20);

        let settings_2 = Settings {
            width: 20,
            height: 20,
            ..Default::default()
        };
        assert_eq!(settings_2.width, 20);
        assert_eq!(settings_2.height, 20);
    }
}
