use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

use crate::models::{cube::Cube, rotation::Rotation};

#[derive(Debug, Clone)]
pub struct CubeWithRotationLog {
    pub cube: Cube,
    pub log: Vec<Rotation>,
}
pub fn solve(cube: Cube, max_recursions: usize) -> Option<CubeWithRotationLog> {
    let solutions: Arc<RwLock<HashMap<usize, Mutex<CubeWithRotationLog>>>> =
        Arc::new(RwLock::new(HashMap::new()));
    let arc_solution = Arc::clone(&solutions);
    let count = recursive_solver(cube, Vec::new(), max_recursions, arc_solution);
    println!("count: {}", count);

    let hmap = solutions.read().expect("RwLock poisoned");

    println!("Number of solutions found: {}", hmap.len());

    let key = if let Some(k) = hmap.keys().min() {
        k
    } else {
        return None;
    };

    let a = hmap.get(key).unwrap();

    let b = if let Ok(c) = a.to_owned().lock() {
        c
    } else {
        return None;
    };

    let c = b.cube;
    let d = &b.log;

    Some(CubeWithRotationLog {
        cube: c,
        log: d.to_vec(),
    })
}

fn recursive_solver(
    cube: Cube,
    rotation_log: Vec<Rotation>,
    recursions_left: usize,
    solutions: Arc<RwLock<HashMap<usize, Mutex<CubeWithRotationLog>>>>,
) -> u32 {
    // If the cube is correct, break early
    if cube.is_correct() {
        let solution = CubeWithRotationLog {
            cube: cube,
            log: rotation_log.clone(),
        };

        solutions
            .write()
            .expect("RwLock poisoned")
            .entry(solution.log.len())
            .or_insert_with(|| Mutex::new(solution));

        return 1;
    }

    let map = solutions.read().expect("RwLock poisoned");

    // If an existing solution has better or equal number of rotations, break early
    if map.contains_key(&rotation_log.len()) {
        return 1;
    }

    drop(map);

    // If we are out of recursions, break early
    if recursions_left == 0 {
        return 1;
    }

    let mut count = 1;

    for &rotation in Rotation::iterator() {
        let mut rotations = rotation_log.clone();

        // If an earlier branch would yield the same result with less rotations in the opposite direction, skip rotation
        match rotations.last() {
            Some(&prev) => {
                if !Cube::allowed_rotation(prev, rotation) {
                    continue;
                }
            }
            _ => (),
        }

        if rotation_log.ends_with(&[rotation]) {
            continue;
        }

        rotations.push(rotation);

        // Look deeper
        count += recursive_solver(
            cube.rotate(rotation),
            rotations,
            recursions_left - 1,
            Arc::clone(&solutions),
        );
    }

    count
}
