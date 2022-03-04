mod location;
mod orientations;
mod position;
mod puzzle;
mod settings;
mod utils;

use js_sys::Math::{floor, random};
use location::*;
use orientations::*;
use position::*;
use puzzle::*;
use settings::*;
use std::collections::HashMap;
use utils::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct WordSearch<'a> {
    words: Vec<String>,
    settings: Settings,
    max_word_len: i32,
    pub grid_fill_percentage: f64,
    pub puzzle: Puzzle,
    pub placed_words: HashMap<String, Location<'a>>,
    pub unplaced_words: Vec<String>,
}

impl<'a> WordSearch<'a> {
    fn fill_puzzle(&mut self) -> Option<Vec<Vec<char>>> {
        let mut puzzle =
            vec![vec![' '; self.settings.width as usize]; self.settings.height as usize];
        let mut i: usize = 0;
        while i < self.words.len() {
            if self.place_word_in_puzzle(&self.words[i], &mut puzzle) {
                i += 1;
            } else {
                self.puzzle.words_not_placed.push(self.words[i].clone());
                return None;
            }
        }
        Some(puzzle)
    }

    fn place_word_in_puzzle(&self, word: &str, puzzle: &mut Vec<Vec<char>>) -> bool {
        let locations = self.find_best_locations(word, puzzle);
        if locations.len() == 0 {
            return false;
        }
        let selected_location: Location =
            locations[floor(random() * locations.len() as f64) as usize];
        self.place_word(word, selected_location, puzzle);
        true
    }

    fn find_best_locations(&'a self, word: &'a str, puzzle: &Vec<Vec<char>>) -> Vec<Location> {
        let mut locations: Vec<Location> = Vec::new();
        let mut pruned_locations: Vec<Location> = Vec::new();
        let height: i32 = self.settings.height;
        let width: i32 = self.settings.width;
        let word_len = word.len() as i32;
        let mut max_overlap: i32 = 0;

        for orientation in self.settings.orientations.iter() {
            let skip = skip_orientation(orientation);
            let check = check_orientation(orientation);
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            while y < height {
                if check(x, y, height, width, word_len) {
                    let overlap: i32 = self.calculate_overlap(word, puzzle, x, y, orientation);
                    if overlap >= max_overlap {
                        max_overlap = overlap;
                        locations.push(Location::from(x, y, overlap, orientation, word));
                    }
                    x += 1;
                    if x >= width {
                        x = 0;
                        y += 1;
                    }
                } else {
                    let next_possible: Position = skip(x, y, word_len);
                    x = next_possible.x;
                    y = next_possible.y;
                }
            }
        }

        for location in locations.iter() {
            if location.overlap >= max_overlap {
                pruned_locations.push(location.clone());
            }
        }
        pruned_locations
    }

    fn calculate_overlap(
        &self,
        word: &str,
        puzzle: &Vec<Vec<char>>,
        x: i32,
        y: i32,
        orientation: &Orientation,
    ) -> i32 {
        let next_orientation = get_orientation(orientation);
        let mut overlap: i32 = 0;
        let mut i: i32 = 0;
        while i < word.len() as i32 {
            let position: Position = next_orientation(x, y, i);
            match puzzle[position.y as usize][position.x as usize] {
                ' ' => {}
                c => {
                    if c == word.chars().nth(i as usize).unwrap() {
                        overlap += 1;
                    } else {
                        return -1;
                    }
                }
            }
            i += 1;
        }
        overlap
    }

    fn place_word(&self, word: &str, location: Location, puzzle: &mut Vec<Vec<char>>) {
        let orientation = location.orientation;
        let next_orientation = get_orientation(orientation);
        let mut i: i32 = 0;
        while i < word.len() as i32 {
            let position: Position = next_orientation(location.x, location.y, i);
            match puzzle[position.y as usize][position.x as usize] {
                ' ' => {
                    puzzle[position.y as usize][position.x as usize] =
                        word.chars().nth(i as usize).unwrap();
                }
                _ => {}
            }
            i += 1;
        }
    }

    fn fill_blanks(&self, puzzle: &mut Vec<Vec<char>>) -> (i32, Vec<Vec<char>>) {
        let mut extra_letters_count: i32 = 0;
        let mut i: usize = 0;
        while i < puzzle.len() {
            let mut j: usize = 0;
            while j < puzzle[i as usize].len() {
                if puzzle[i][j] == ' ' {
                    puzzle[i][j] = rand_char();
                    extra_letters_count += 1;
                }
                j += 1;
            }
            i += 1;
        }
        (extra_letters_count, puzzle.clone())
    }

    fn update_size(&mut self, w: i32, h: i32) {
        let m = self.max_word_len;
        let width = if m as i32 > w { m as i32 } else { w };
        let height = if m as i32 > h { m as i32 } else { h };

        self.settings.set_width(width);
        self.settings.set_height(height);
        self.puzzle = Puzzle::new(self.settings.width, self.settings.height);
        self.grid_fill_percentage = 0.0;
    }

    pub fn new(words: &Vec<String>, w: i32, h: i32) -> WordSearch {
        set_panic_hook();

        if words.len() == 0 {
            panic!("No words provided");
        }

        let mut word_list: Vec<String> = words.clone();
        word_list.sort_by(|a, b| a.len().cmp(&b.len()));
        let m: i32 = word_list.last().unwrap().len() as i32;
        let width = if m as i32 > w { m as i32 } else { w };
        let height = if m as i32 > h { m as i32 } else { h };

        let word_search = WordSearch {
            words: words.clone(),
            settings: Settings {
                width,
                height,
                ..Settings::default()
            },
            puzzle: Puzzle::new(width, height),
            grid_fill_percentage: 0.0,
            max_word_len: m,
            placed_words: HashMap::new(),
            unplaced_words: Vec::new(),
        };
        word_search
    }

    pub fn create(&mut self) -> &mut WordSearch<'a> {
        set_panic_hook();

        let mut puzzle: Option<Vec<Vec<char>>> = None;
        let mut attempts = 0;
        let mut grid_growths = 0;

        loop {
            attempts += 1;

            if attempts < self.settings.max_attempts {
                self.puzzle.words_not_placed = Vec::new();
                puzzle = self.fill_puzzle();
            }

            match puzzle {
                None => {
                    grid_growths += 1;

                    if grid_growths > self.settings.max_grid_growth {
                        self.puzzle.add_errors(&format!(
                            "No valid {}x{} grid found and not allowed to grow more",
                            self.settings.width, self.settings.height
                        ));
                        return self;
                    }

                    // Trying a bigger grid after nth attempts
                    self.update_size(self.settings.width + 1, self.settings.height + 1);
                    attempts = 0;
                }
                Some(mut p) => {
                    let (extra_letters_count, p) = self.fill_blanks(&mut p);
                    self.grid_fill_percentage = (1.0
                        - extra_letters_count as f64
                            / (self.settings.width * self.settings.height) as f64)
                        * 100.0;
                    self.puzzle.puzzle = p;
                    // self.solve();
                    return self;
                }
            }
        }
    }

    // fn solve(&mut self) {
    //     // Solving the puzzle
    //     let p_copy = self.puzzle.puzzle.clone();
    //     let mut i = 0;
    //     while i < self.words.len() {
    //         let word = self.words[i].as_str();
    //         let locations = self.find_best_locations(word, &p_copy);
    //         if locations.len() != 0 {
    //             let w_copy = String::from(word);
    //             let mut location = locations.iter_mut().as_slice();
    //             location[0].word = word;
    //             self.placed_words.insert(w_copy, location[0]);
    //         } else {
    //             self.unplaced_words.push(String::from(word));
    //         }
    //         i += 1;
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_search() {
        let words: Vec<String> = vec![
            String::from("hello"),
            String::from("world"),
            String::from("test"),
            String::from("mark"),
        ];
        let word_search = WordSearch::new(&words, 4, 4);
        assert_eq!(word_search.words.len(), 4);
        assert_eq!(word_search.settings.width, 5);
        assert_eq!(word_search.settings.height, 5);
    }
}
