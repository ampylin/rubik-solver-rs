use std::slice::Iter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    LeftUp,
    LeftUp2,
    LeftDown,
    TopLeft,
    TopLeft2,
    TopRight,
    FrontClockwise,
    FrontClockwise2,
    FrontCounterclockwise,
}

impl Rotation {
    pub fn iterator() -> Iter<'static, Rotation> {
        static ROTATION: [Rotation; 9] = [
            Rotation::LeftUp,
            Rotation::LeftUp2,
            Rotation::LeftDown,
            Rotation::TopLeft,
            Rotation::TopLeft2,
            Rotation::TopRight,
            Rotation::FrontClockwise,
            Rotation::FrontClockwise2,
            Rotation::FrontCounterclockwise,
        ];

        ROTATION.iter()
    }
}
