use std::fmt;

use super::color::Color;

/**
 * A representation of a side of a 2-by-2 cube.
 */
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Side {
    pub top_left: Color,
    pub top_right: Color,
    pub bottom_left: Color,
    pub bottom_right: Color,
}

impl Side {
    pub fn rotate_clockwise(&self) -> Side {
        Side {
            top_left: self.bottom_left,
            top_right: self.top_left,
            bottom_left: self.bottom_right,
            bottom_right: self.top_right,
        }
    }
    pub fn rotate_counterclockwise(&self) -> Side {
        Side {
            top_left: self.top_right,
            top_right: self.bottom_right,
            bottom_left: self.top_left,
            bottom_right: self.bottom_left,
        }
    }

    pub fn is_one_color(&self) -> bool {
        self.top_left == self.top_right
            && self.top_right == self.bottom_left
            && self.bottom_left == self.bottom_right
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}\n{}{}",
            self.top_left, self.top_right, self.bottom_left, self.bottom_right
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_clockwise() {
        assert_eq!(
            Side {
                top_left: Color::Green,
                top_right: Color::Yellow,
                bottom_left: Color::Red,
                bottom_right: Color::Blue,
            }
            .rotate_clockwise(),
            Side {
                top_left: Color::Red,
                top_right: Color::Green,
                bottom_left: Color::Blue,
                bottom_right: Color::Yellow,
            }
        );
    }

    #[test]
    fn test_rotate_counterclockwise() {
        assert_eq!(
            Side {
                top_left: Color::Red,
                top_right: Color::Green,
                bottom_left: Color::Blue,
                bottom_right: Color::Yellow,
            }
            .rotate_counterclockwise(),
            Side {
                top_left: Color::Green,
                top_right: Color::Yellow,
                bottom_left: Color::Red,
                bottom_right: Color::Blue,
            }
        );
    }
}
