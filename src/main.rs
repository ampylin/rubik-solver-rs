mod models;
mod solvers;
use models::{color::Color, cube::Cube, side::Side};

use crate::solvers::solver::solve;

fn main() {
    // Example of how to create a N move cube
    // let original_cube = Cube {
    //     front: Side {
    //         top_left: Color::Red,
    //         top_right: Color::Red,
    //         bottom_left: Color::Red,
    //         bottom_right: Color::Red,
    //     },
    //     right: Side {
    //         top_left: Color::White,
    //         top_right: Color::White,
    //         bottom_left: Color::White,
    //         bottom_right: Color::White,
    //     },
    //     bottom: Side {
    //         top_left: Color::Blue,
    //         top_right: Color::Blue,
    //         bottom_left: Color::Blue,
    //         bottom_right: Color::Blue,
    //     },
    //     left: Side {
    //         top_left: Color::Yellow,
    //         top_right: Color::Yellow,
    //         bottom_left: Color::Yellow,
    //         bottom_right: Color::Yellow,
    //     },
    //     top: Side {
    //         top_left: Color::Green,
    //         top_right: Color::Green,
    //         bottom_left: Color::Green,
    //         bottom_right: Color::Green,
    //     },
    //     back: Side {
    //         top_left: Color::Orange,
    //         top_right: Color::Orange,
    //         bottom_left: Color::Orange,
    //         bottom_right: Color::Orange,
    //     },
    // }
    // .rotate(Rotation::FrontClockwise)
    // .rotate(Rotation::LeftUp)
    // .rotate(Rotation::LeftUp)
    // .rotate(Rotation::TopLeft)
    // .rotate(Rotation::LeftUp)
    // .rotate(Rotation::FrontCounterclockwise)
    // .rotate(Rotation::LeftDown)
    // .rotate(Rotation::FrontClockwise)
    // .rotate(Rotation::TopRight)
    // .rotate(Rotation::TopRight)
    // .rotate(Rotation::FrontCounterclockwise)
    // .rotate(Rotation::FrontCounterclockwise);

    let original_cube = Cube {
        back: Side {
            top_left: Color::Red,
            top_right: Color::Red,
            bottom_left: Color::Red,
            bottom_right: Color::Red,
        },
        left: Side {
            top_left: Color::White,
            top_right: Color::Green,
            bottom_left: Color::White,
            bottom_right: Color::White,
        },
        bottom: Side {
            top_left: Color::Blue,
            top_right: Color::Blue,
            bottom_left: Color::Blue,
            bottom_right: Color::Blue,
        },
        right: Side {
            top_left: Color::Green,
            top_right: Color::Yellow,
            bottom_left: Color::Yellow,
            bottom_right: Color::Yellow,
        },
        top: Side {
            bottom_right: Color::White,
            bottom_left: Color::Yellow,
            top_right: Color::Green,
            top_left: Color::Green,
        },
        front: Side {
            bottom_right: Color::Orange,
            bottom_left: Color::Orange,
            top_right: Color::Orange,
            top_left: Color::Orange,
        },
    };

    let remaining_depth = 10;

    match solve(original_cube, remaining_depth) {
        Some(solution) => {
            println!("The cube was solved");
            solution.cube.print();
            println!("{:?}", solution.log);
        }
        None => println!("No solution found for with depth {}", remaining_depth),
    }
}
