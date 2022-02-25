#[derive(Debug, Clone, Copy)]
pub struct Position {
    // The col where the word starts
    pub x: i32,
    // The row where the word starts
    pub y: i32,
}

impl Position {
    // Constructor
    pub fn from(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_position() {
        let p = Position::from(1, 2);
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 2);
    }
}