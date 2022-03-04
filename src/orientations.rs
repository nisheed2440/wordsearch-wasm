use crate::position::Position;

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Horizontal,
    Vertical,
    Diagonal,
    DiagonalUp,
}

pub fn get_orientation(orientation: &Orientation) -> Box<dyn Fn(i32, i32, i32) -> Position> {
    match orientation {
        Orientation::Horizontal => Box::new(|x, y, i| Position::from(x + i, y)),
        Orientation::Vertical => Box::new(|x, y, i| Position::from(x, y + i)),
        Orientation::Diagonal => Box::new(|x, y, i| Position::from(x + i, y + i)),
        Orientation::DiagonalUp => Box::new(|x, y, i| Position::from(x + i, y - i)),
    }
}

pub fn check_orientation(
    orientation: &Orientation,
) -> Box<dyn Fn(i32, i32, i32, i32, i32) -> bool> {
    match orientation {
        Orientation::Horizontal => Box::new(|x, _y, _h, w, l| w >= x + l),
        Orientation::Vertical => Box::new(|_x, y, h, _w, l| h >= y + l),
        Orientation::Diagonal => Box::new(|x, y, h, w, l| w >= x + l && h >= y + l),
        Orientation::DiagonalUp => Box::new(|x, y, _h, w, l| w >= x + l && y + 1 >= l),
    }
}

pub fn skip_orientation(orientation: &Orientation) -> Box<dyn Fn(i32, i32, i32) -> Position> {
    match orientation {
        Orientation::Horizontal => Box::new(|x, y, _l| Position::from(x, y + 1)),
        Orientation::Vertical => Box::new(|_x, y, _l| Position::from(0, y + 100)),
        Orientation::Diagonal => Box::new(|_x, y, _l| Position::from(0, y + 1)),
        Orientation::DiagonalUp => {
            Box::new(|_x, y, l| Position::from(0, if y < l - 1 { l - 1 } else { y + 1 }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_orientation() {
        let orientation_h = Orientation::Horizontal;
        let position_h: Position = get_orientation(&orientation_h)(1, 2, 3);
        assert_eq!(position_h.x, 4);
        assert_eq!(position_h.y, 2);

        let orientation_v = Orientation::Vertical;
        let position_v: Position = get_orientation(&orientation_v)(1, 2, 3);
        assert_eq!(position_v.x, 1);
        assert_eq!(position_v.y, 5);

        let orientation_d = Orientation::Diagonal;
        let position_d: Position = get_orientation(&orientation_d)(1, 2, 3);
        assert_eq!(position_d.x, 4);
        assert_eq!(position_d.y, 5);
    }

    #[test]
    fn test_check_orientation() {
        let orientation_h = Orientation::Horizontal;
        assert_eq!(check_orientation(&orientation_h)(1, 2, 3, 5, 4), true);
        assert_eq!(check_orientation(&orientation_h)(1, 2, 3, 5, 3), true);

        let orientation_v = Orientation::Vertical;
        assert_eq!(check_orientation(&orientation_v)(1, 1, 5, 5, 4), true);
        assert_eq!(check_orientation(&orientation_v)(1, 1, 5, 5, 3), true);

        let orientation_d = Orientation::Diagonal;
        assert_eq!(check_orientation(&orientation_d)(1, 1, 5, 5, 4), true);
    }

    #[test]
    fn test_skip_orientation() {
        let orientation_h = Orientation::Horizontal;
        let position_h: Position = skip_orientation(&orientation_h)(1, 2, 3);
        assert_eq!(position_h.x, 1);
        assert_eq!(position_h.y, 3);

        let orientation_v = Orientation::Vertical;
        let position_v: Position = skip_orientation(&orientation_v)(1, 2, 3);
        assert_eq!(position_v.x, 0);
        assert_eq!(position_v.y, 102);

        let orientation_d = Orientation::Diagonal;
        let position_d: Position = skip_orientation(&orientation_d)(1, 2, 3);
        assert_eq!(position_d.x, 0);
        assert_eq!(position_d.y, 3);
    }
}
