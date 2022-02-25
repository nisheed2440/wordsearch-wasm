use crate::orientations::*;

#[derive(Debug, Clone, Copy)]
pub struct Location<'a> {
    // The col where the word starts
    pub x: i32,
    // The row where the word starts
    pub y: i32,
    // The number of overlaps the word has
    pub overlap: i32,
    // The orientation of the word
    pub orientation: &'a Orientation,
    // The word itself
    pub word: &'a str,
}

impl<'a> Location<'a> {
    // Constructor
    pub fn from(x: i32, y: i32, overlap: i32, orientation: &'a Orientation, word: &'a str) -> Location<'a> {
        Location {
            x,
            y,
            overlap,
            orientation,
            word,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_location() {
        let x = 1;
        let y = 2;
        let overlap = 3;
        let orientation = Orientation::Horizontal;
        let word = String::from("test");
        let location = Location::from(x, y, overlap, &orientation, &word);
        assert_eq!(location.x, x);
        assert_eq!(location.y, y);
        assert_eq!(location.overlap, overlap);
        assert_eq!(location.word, word);
    }
}
