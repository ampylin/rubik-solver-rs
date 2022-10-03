use super::{rotation::Rotation, side::Side};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Cube {
    pub front: Side,
    pub right: Side,
    pub bottom: Side,
    pub left: Side,
    pub top: Side,
    pub back: Side,
}

impl Cube {
    pub fn rotate(&self, rotation: Rotation) -> Cube {
        match rotation {
            Rotation::LeftUp => self.move_left_up(),
            Rotation::LeftUp2 => self.move_left_up().move_left_up(),
            Rotation::LeftDown => self.move_left_down(),
            Rotation::TopLeft => self.move_top_left(),
            Rotation::TopLeft2 => self.move_top_left().move_top_left(),
            Rotation::TopRight => self.move_top_right(),
            Rotation::FrontClockwise => self.move_front_clockwise(),
            Rotation::FrontClockwise2 => self.move_front_clockwise().move_front_clockwise(),
            Rotation::FrontCounterclockwise => self.move_front_counterclockwise(),
        }
    }

    pub fn allowed_rotation(prev: Rotation, next: Rotation) -> bool {
        match prev {
            Rotation::LeftUp => match next {
                Rotation::LeftUp => false,
                Rotation::LeftUp2 => false,
                Rotation::LeftDown => false,
                _ => true,
            },
            Rotation::LeftUp2 => match next {
                Rotation::LeftUp => false,
                Rotation::LeftUp2 => false,
                Rotation::LeftDown => false,
                _ => true,
            },
            Rotation::LeftDown => match next {
                Rotation::LeftUp => false,
                Rotation::LeftUp2 => false,
                Rotation::LeftDown => false,
                _ => true,
            },
            Rotation::TopLeft => match next {
                Rotation::TopLeft => false,
                Rotation::TopLeft2 => false,
                Rotation::TopRight => false,
                _ => true,
            },
            Rotation::TopLeft2 => match next {
                Rotation::TopLeft => false,
                Rotation::TopLeft2 => false,
                Rotation::TopRight => false,
                _ => true,
            },
            Rotation::TopRight => match next {
                Rotation::TopLeft => false,
                Rotation::TopLeft2 => false,
                Rotation::TopRight => false,
                _ => true,
            },
            Rotation::FrontClockwise => match next {
                Rotation::FrontClockwise => false,
                Rotation::FrontClockwise2 => false,
                Rotation::FrontCounterclockwise => false,
                _ => true,
            },
            Rotation::FrontClockwise2 => match next {
                Rotation::FrontClockwise => false,
                Rotation::FrontClockwise2 => false,
                Rotation::FrontCounterclockwise => false,
                _ => true,
            },
            Rotation::FrontCounterclockwise => match next {
                Rotation::FrontClockwise => false,
                Rotation::FrontClockwise2 => false,
                Rotation::FrontCounterclockwise => false,
                _ => true,
            },
        }
    }

    fn move_left_up(&self) -> Cube {
        Cube {
            front: Side {
                top_left: self.bottom.top_left,
                top_right: self.front.top_right,
                bottom_left: self.bottom.bottom_left,
                bottom_right: self.front.bottom_right,
            },
            right: self.right,
            bottom: Side {
                top_left: self.back.top_left,
                top_right: self.bottom.top_right,
                bottom_left: self.back.bottom_left,
                bottom_right: self.bottom.bottom_right,
            },
            left: self.left.rotate_counterclockwise(),
            top: Side {
                top_left: self.front.top_left,
                top_right: self.top.top_right,
                bottom_left: self.front.bottom_left,
                bottom_right: self.top.bottom_right,
            },
            back: Side {
                top_left: self.top.top_left,
                top_right: self.back.top_right,
                bottom_left: self.top.bottom_left,
                bottom_right: self.back.bottom_right,
            },
        }
    }

    fn move_left_down(&self) -> Cube {
        Cube {
            front: Side {
                top_left: self.top.top_left,
                top_right: self.front.top_right,
                bottom_left: self.top.bottom_left,
                bottom_right: self.front.bottom_right,
            },
            right: self.right,
            bottom: Side {
                top_left: self.front.top_left,
                top_right: self.bottom.top_right,
                bottom_left: self.front.bottom_left,
                bottom_right: self.bottom.bottom_right,
            },
            left: self.left.rotate_clockwise(),
            top: Side {
                top_left: self.back.top_left,
                top_right: self.top.top_right,
                bottom_left: self.back.bottom_left,
                bottom_right: self.top.bottom_right,
            },
            back: Side {
                top_left: self.bottom.top_left,
                top_right: self.back.top_right,
                bottom_left: self.bottom.bottom_left,
                bottom_right: self.back.bottom_right,
            },
        }
    }

    fn move_top_left(&self) -> Cube {
        Cube {
            front: Side {
                top_left: self.right.top_left,
                top_right: self.right.top_right,
                bottom_left: self.front.bottom_left,
                bottom_right: self.front.bottom_right,
            },
            right: Side {
                top_left: self.back.bottom_right,
                top_right: self.back.bottom_left,
                bottom_left: self.right.bottom_left,
                bottom_right: self.right.bottom_right,
            },
            bottom: self.bottom,
            left: Side {
                top_left: self.front.top_left,
                top_right: self.front.top_right,
                bottom_left: self.left.bottom_left,
                bottom_right: self.left.bottom_right,
            },
            top: self.top.rotate_clockwise(),
            back: Side {
                top_left: self.back.top_left,
                top_right: self.back.top_right,
                bottom_left: self.left.top_right,
                bottom_right: self.left.top_left,
            },
        }
    }

    fn move_top_right(&self) -> Cube {
        Cube {
            front: Side {
                top_left: self.left.top_left,
                top_right: self.left.top_right,
                bottom_left: self.front.bottom_left,
                bottom_right: self.front.bottom_right,
            },
            right: Side {
                top_left: self.front.top_left,
                top_right: self.front.top_right,
                bottom_left: self.right.bottom_left,
                bottom_right: self.right.bottom_right,
            },
            bottom: self.bottom,
            left: Side {
                top_left: self.back.bottom_right,
                top_right: self.back.bottom_left,
                bottom_left: self.left.bottom_left,
                bottom_right: self.left.bottom_right,
            },
            top: self.top.rotate_counterclockwise(),
            back: Side {
                top_left: self.back.top_left,
                top_right: self.back.top_right,
                bottom_left: self.right.top_right,
                bottom_right: self.right.top_left,
            },
        }
    }

    fn move_front_clockwise(&self) -> Cube {
        Cube {
            front: self.front.rotate_clockwise(),
            right: Side {
                top_left: self.top.bottom_left,
                top_right: self.right.top_right,
                bottom_left: self.top.bottom_right,
                bottom_right: self.right.bottom_right,
            },
            bottom: Side {
                top_left: self.right.bottom_left,
                top_right: self.right.top_left,
                bottom_left: self.bottom.bottom_left,
                bottom_right: self.bottom.bottom_right,
            },
            left: Side {
                top_left: self.left.top_left,
                top_right: self.bottom.top_left,
                bottom_left: self.left.bottom_left,
                bottom_right: self.bottom.top_right,
            },
            top: Side {
                top_left: self.top.top_left,
                top_right: self.top.top_right,
                bottom_left: self.left.bottom_right,
                bottom_right: self.left.top_right,
            },
            back: self.back,
        }
    }

    fn move_front_counterclockwise(&self) -> Cube {
        Cube {
            front: self.front.rotate_counterclockwise(),
            right: Side {
                top_left: self.bottom.top_right,
                top_right: self.right.top_right,
                bottom_left: self.bottom.top_left,
                bottom_right: self.right.bottom_right,
            },
            bottom: Side {
                top_left: self.left.top_right,
                top_right: self.left.bottom_right,
                bottom_left: self.bottom.bottom_left,
                bottom_right: self.bottom.bottom_right,
            },
            left: Side {
                top_left: self.left.top_left,
                top_right: self.top.bottom_right,
                bottom_left: self.left.bottom_left,
                bottom_right: self.top.bottom_left,
            },
            top: Side {
                top_left: self.top.top_left,
                top_right: self.top.top_right,
                bottom_left: self.right.top_left,
                bottom_right: self.right.bottom_left,
            },
            back: self.back,
        }
    }

    pub fn is_correct(&self) -> bool {
        self.front.is_one_color()
            && self.right.is_one_color()
            && self.bottom.is_one_color()
            && self.left.is_one_color()
            && self.top.is_one_color()
            && self.back.is_one_color()
    }

    pub fn print(&self) {
        println!("");
        println!("  {}{}", self.back.top_left, self.back.top_right);

        println!("  {}{}", self.back.bottom_left, self.back.bottom_right);

        println!("  {}{}", self.top.top_left, self.top.top_right);

        println!("  {}{}", self.top.bottom_left, self.top.bottom_right);

        println!(
            "{}{}{}{}{}{}",
            self.left.top_left,
            self.left.top_right,
            self.front.top_left,
            self.front.top_right,
            self.right.top_left,
            self.right.top_right
        );

        println!(
            "{}{}{}{}{}{}",
            self.left.bottom_left,
            self.left.bottom_right,
            self.front.bottom_left,
            self.front.bottom_right,
            self.right.bottom_left,
            self.right.bottom_right
        );

        println!("  {}{}", self.bottom.top_left, self.bottom.top_right);

        println!("  {}{}", self.bottom.bottom_left, self.bottom.bottom_right);
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::super::color::Color;
    use super::*;

    fn create_test_cube() -> Cube {
        Cube {
            front: Side {
                top_left: Color::Green,
                top_right: Color::Yellow,
                bottom_left: Color::Blue,
                bottom_right: Color::White,
            },
            right: Side {
                top_left: Color::Green,
                top_right: Color::Blue,
                bottom_left: Color::Green,
                bottom_right: Color::Red,
            },
            bottom: Side {
                top_left: Color::Orange,
                top_right: Color::Orange,
                bottom_left: Color::Red,
                bottom_right: Color::Blue,
            },
            left: Side {
                top_left: Color::Yellow,
                top_right: Color::Red,
                bottom_left: Color::White,
                bottom_right: Color::Yellow,
            },
            top: Side {
                top_left: Color::Blue,
                top_right: Color::White,
                bottom_left: Color::Yellow,
                bottom_right: Color::Orange,
            },
            back: Side {
                top_left: Color::Green,
                top_right: Color::White,
                bottom_left: Color::Red,
                bottom_right: Color::Orange,
            },
        }
    }

    #[test]
    fn test_rotate_left_up() {
        assert_eq!(
            create_test_cube().rotate(Rotation::LeftUp),
            Cube {
                front: Side {
                    top_left: Color::Orange,
                    top_right: Color::Yellow,
                    bottom_left: Color::Red,
                    bottom_right: Color::White,
                },
                right: Side {
                    top_left: Color::Green,
                    top_right: Color::Blue,
                    bottom_left: Color::Green,
                    bottom_right: Color::Red,
                },
                bottom: Side {
                    top_left: Color::Green,
                    top_right: Color::Orange,
                    bottom_left: Color::Red,
                    bottom_right: Color::Blue,
                },
                left: Side {
                    top_left: Color::Red,
                    top_right: Color::Yellow,
                    bottom_left: Color::Yellow,
                    bottom_right: Color::White,
                },
                top: Side {
                    top_left: Color::Green,
                    top_right: Color::White,
                    bottom_left: Color::Blue,
                    bottom_right: Color::Orange,
                },
                back: Side {
                    top_left: Color::Blue,
                    top_right: Color::White,
                    bottom_left: Color::Yellow,
                    bottom_right: Color::Orange,
                },
            }
        );
    }

    #[test]
    fn test_rotate_left_down() {
        assert_eq!(
            Cube {
                front: Side {
                    top_left: Color::Orange,
                    top_right: Color::Yellow,
                    bottom_left: Color::Red,
                    bottom_right: Color::White,
                },
                right: Side {
                    top_left: Color::Green,
                    top_right: Color::Blue,
                    bottom_left: Color::Green,
                    bottom_right: Color::Red,
                },
                bottom: Side {
                    top_left: Color::Green,
                    top_right: Color::Orange,
                    bottom_left: Color::Red,
                    bottom_right: Color::Blue,
                },
                left: Side {
                    top_left: Color::Red,
                    top_right: Color::Yellow,
                    bottom_left: Color::Yellow,
                    bottom_right: Color::White,
                },
                top: Side {
                    top_left: Color::Green,
                    top_right: Color::White,
                    bottom_left: Color::Blue,
                    bottom_right: Color::Orange,
                },
                back: Side {
                    top_left: Color::Blue,
                    top_right: Color::White,
                    bottom_left: Color::Yellow,
                    bottom_right: Color::Orange,
                },
            }
            .rotate(Rotation::LeftDown),
            create_test_cube()
        );
    }

    #[test]
    fn test_rotate_top_left() {
        assert_eq!(
            create_test_cube().rotate(Rotation::TopLeft),
            Cube {
                front: Side {
                    top_left: Color::Green,
                    top_right: Color::Blue,
                    bottom_left: Color::Blue,
                    bottom_right: Color::White,
                },
                right: Side {
                    top_left: Color::Orange,
                    top_right: Color::Red,
                    bottom_left: Color::Green,
                    bottom_right: Color::Red,
                },
                bottom: Side {
                    top_left: Color::Orange,
                    top_right: Color::Orange,
                    bottom_left: Color::Red,
                    bottom_right: Color::Blue,
                },
                left: Side {
                    top_left: Color::Green,
                    top_right: Color::Yellow,
                    bottom_left: Color::White,
                    bottom_right: Color::Yellow,
                },
                top: Side {
                    top_left: Color::Yellow,
                    top_right: Color::Blue,
                    bottom_left: Color::Orange,
                    bottom_right: Color::White,
                },
                back: Side {
                    top_left: Color::Green,
                    top_right: Color::White,
                    bottom_left: Color::Red,
                    bottom_right: Color::Yellow,
                },
            }
        );
    }

    #[test]
    fn test_rotate_top_right() {
        assert_eq!(
            Cube {
                front: Side {
                    top_left: Color::Green,
                    top_right: Color::Blue,
                    bottom_left: Color::Blue,
                    bottom_right: Color::White,
                },
                right: Side {
                    top_left: Color::Orange,
                    top_right: Color::Red,
                    bottom_left: Color::Green,
                    bottom_right: Color::Red,
                },
                bottom: Side {
                    top_left: Color::Orange,
                    top_right: Color::Orange,
                    bottom_left: Color::Red,
                    bottom_right: Color::Blue,
                },
                left: Side {
                    top_left: Color::Green,
                    top_right: Color::Yellow,
                    bottom_left: Color::White,
                    bottom_right: Color::Yellow,
                },
                top: Side {
                    top_left: Color::Yellow,
                    top_right: Color::Blue,
                    bottom_left: Color::Orange,
                    bottom_right: Color::White,
                },
                back: Side {
                    top_left: Color::Green,
                    top_right: Color::White,
                    bottom_left: Color::Red,
                    bottom_right: Color::Yellow,
                },
            }
            .rotate(Rotation::TopRight),
            create_test_cube()
        );
    }

    #[test]
    fn test_rotate_front_clockwise() {
        assert_eq!(
            create_test_cube().rotate(Rotation::FrontClockwise),
            Cube {
                front: Side {
                    top_left: Color::Blue,
                    top_right: Color::Green,
                    bottom_left: Color::White,
                    bottom_right: Color::Yellow,
                },
                right: Side {
                    top_left: Color::Yellow,
                    top_right: Color::Blue,
                    bottom_left: Color::Orange,
                    bottom_right: Color::Red,
                },
                bottom: Side {
                    top_left: Color::Green,
                    top_right: Color::Green,
                    bottom_left: Color::Red,
                    bottom_right: Color::Blue,
                },
                left: Side {
                    top_left: Color::Yellow,
                    top_right: Color::Orange,
                    bottom_left: Color::White,
                    bottom_right: Color::Orange,
                },
                top: Side {
                    top_left: Color::Blue,
                    top_right: Color::White,
                    bottom_left: Color::Yellow,
                    bottom_right: Color::Red,
                },
                back: Side {
                    top_left: Color::Green,
                    top_right: Color::White,
                    bottom_left: Color::Red,
                    bottom_right: Color::Orange,
                },
            }
        );
    }

    #[test]
    fn test_rotate_front_counterclockwise() {
        assert_eq!(
            Cube {
                front: Side {
                    top_left: Color::Blue,
                    top_right: Color::Green,
                    bottom_left: Color::White,
                    bottom_right: Color::Yellow,
                },
                right: Side {
                    top_left: Color::Yellow,
                    top_right: Color::Blue,
                    bottom_left: Color::Orange,
                    bottom_right: Color::Red,
                },
                bottom: Side {
                    top_left: Color::Green,
                    top_right: Color::Green,
                    bottom_left: Color::Red,
                    bottom_right: Color::Blue,
                },
                left: Side {
                    top_left: Color::Yellow,
                    top_right: Color::Orange,
                    bottom_left: Color::White,
                    bottom_right: Color::Orange,
                },
                top: Side {
                    top_left: Color::Blue,
                    top_right: Color::White,
                    bottom_left: Color::Yellow,
                    bottom_right: Color::Red,
                },
                back: Side {
                    top_left: Color::Green,
                    top_right: Color::White,
                    bottom_left: Color::Red,
                    bottom_right: Color::Orange,
                },
            }
            .rotate(Rotation::FrontCounterclockwise),
            create_test_cube()
        );
    }
}
