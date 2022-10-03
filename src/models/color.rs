use std::fmt;

use colorful::Colorful;
extern crate colorful;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    White,
    Blue,
    Yellow,
    Green,
    Orange,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match self {
            Color::Red => colorful::RGB::new(255, 0, 0),
            Color::White => colorful::RGB::new(255, 255, 255),
            Color::Blue => colorful::RGB::new(50, 150, 255),
            Color::Yellow => colorful::RGB::new(255, 250, 0),
            Color::Green => colorful::RGB::new(0, 220, 0),
            Color::Orange => colorful::RGB::new(255, 150, 0),
            // Color::Red => colorful::Color::Red,
            // Color::White => colorful::Color::White,
            // Color::Blue => colorful::Color::Blue,
            // Color::Yellow => colorful::Color::Yellow,
            // Color::Green => colorful::Color::Green,
            // Color::Orange => colorful::Color::Orange1,
        };
        write!(f, "{}", "\u{2588}".color(color))
    }
}
