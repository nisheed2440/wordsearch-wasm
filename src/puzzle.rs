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
    pub fn new() -> Puzzle {
        Puzzle {
            puzzle: vec![],
            words_not_placed: vec![],
            warnings: vec![],
            errors: vec![],
        }
    }

    pub fn set_puzzle(&mut self, puzzle: Vec<Vec<char>>) {
        self.puzzle = puzzle;
    }

    pub fn set_words_not_placed(&mut self, words_not_placed: Vec<String>) {
        self.words_not_placed = words_not_placed;
    }

    pub fn set_warnings(&mut self, warnings: Vec<String>) {
        self.warnings = warnings;
    }

    pub fn set_errors(&mut self, errors: Vec<String>) {
        self.errors = errors;
    }
}