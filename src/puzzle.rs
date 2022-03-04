pub struct Puzzle {
    // Two dimentional list containing the puzzle
    pub puzzle: Vec<Vec<char>>,
    // List of word not placed in the puzzle
    pub words_not_placed: Vec<String>,
    // List of warnings that occured while creating the puzzle
    //
    // **Note:** Use this to notify the user of any issues
    pub warnings: Vec<String>,
    // List of errors that occured while creating the puzzle
    //
    // **Note:** Check this before printing or viewing the puzzle
    pub errors: Vec<String>,
}

impl Puzzle {
    pub fn new(w: i32, h: i32) -> Puzzle {
        Puzzle {
            puzzle: vec![vec![' '; w as usize]; h as usize],
            words_not_placed: vec![],
            warnings: vec![],
            errors: vec![],
        }
    }

    pub fn place_char(&mut self, character: char, x: i32, y: i32) {
        self.puzzle[x as usize][y as usize] = character;
    }

    pub fn add_words_not_placed(&mut self, word: &str) {
        self.words_not_placed.push(String::from(word));
    }

    pub fn add_warning(&mut self, warning: &str) {
        self.warnings.push(String::from(warning));
    }

    pub fn add_errors(&mut self, error: &str) {
        self.errors.push(String::from(error));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_puzzle() {
        let puzzle = Puzzle::new(5, 5);
        assert_eq!(puzzle.puzzle.len(), 5);
        assert_eq!(puzzle.puzzle[0].len(), 5);
        assert_eq!(puzzle.puzzle[1].len(), 5);
        assert_eq!(puzzle.puzzle[2].len(), 5);
        assert_eq!(puzzle.puzzle[3].len(), 5);
        assert_eq!(puzzle.puzzle[4].len(), 5);
    }

    #[test]
    fn test_place_char() {
        let mut puzzle = Puzzle::new(5, 5);
        puzzle.place_char('a', 1, 1);
        assert_eq!(puzzle.puzzle[1][1], 'a');
    }

    #[test]
    fn test_add_words_not_placed() {
        let mut puzzle = Puzzle::new(5, 5);
        puzzle.add_words_not_placed("test");
        assert_eq!(puzzle.words_not_placed[0], "test");
    }

    #[test]
    fn test_add_warning() {
        let mut puzzle = Puzzle::new(5, 5);
        puzzle.add_warning("test");
        assert_eq!(puzzle.warnings[0], "test");
    }

    #[test]
    fn test_add_errors() {
        let mut puzzle = Puzzle::new(5, 5);
        puzzle.add_errors("test");
        assert_eq!(puzzle.errors[0], "test");
    }
}
