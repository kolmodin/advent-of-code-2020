use std::fs;

use std::collections::HashSet;

use std::cmp::max;
use std::cmp::min;

type Pos = (i32, i32, i32, i32);

type Map = HashSet<Pos>;

fn to_map(input: &str) -> Map {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as i32, y as i32, 0, 0))
        })
        .collect()
}

fn bounds(map: &Map) -> ((i32, i32), (i32, i32), (i32, i32), (i32, i32)) {
    let mut x_max = 0;
    let mut x_min = 0;
    let mut y_max = 0;
    let mut y_min = 0;
    let mut z_max = 0;
    let mut z_min = 0;
    let mut w_min = 0;
    let mut w_max = 0;

    for (x, y, z, w) in map.iter() {
        x_min = *min(&x_min, x);
        x_max = *max(&x_max, x);
        y_min = *min(&y_min, y);
        y_max = *max(&y_max, y);
        z_min = *min(&z_min, z);
        z_max = *max(&z_max, z);
        w_min = *min(&w_min, w);
        w_max = *max(&w_max, w);
    }

    (
        (x_min - 1, x_max + 1),
        (y_min - 1, y_max + 1),
        (z_min - 1, z_max + 1),
        (w_min - 1, w_max + 1),
    )
}

fn neighbors(map: &Map, pos: Pos, is_hypercube: bool) -> i32 {
    let (px, py, pz, pw) = pos;

    let mut count = 0;

    let (w_min, w_max) = if is_hypercube {
        (pw - 1, pw + 1)
    } else {
        (0, 0)
    };

    for w in w_min..=w_max {
        for z in (pz - 1)..=(pz + 1) {
            for y in (py - 1)..=(py + 1) {
                for x in (px - 1)..=(px + 1) {
                    if w == pw && z == pz && y == py && x == px {
                        continue;
                    }
                    if map.get(&(x, y, z, w)).is_some() {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn evolve(map: &Map, is_hypercube: bool) -> Map {
    let ((x_min, x_max), (y_min, y_max), (z_min, z_max), (w_min, w_max)) = bounds(map);
    let (w_min, w_max) = if is_hypercube { (w_min, w_max) } else { (0, 0) };

    let mut newmap = HashSet::new();

    for w in w_min..=w_max {
        for z in z_min..=z_max {
            for y in y_min..=y_max {
                for x in x_min..=x_max {
                    let count = neighbors(map, (x, y, z, w), is_hypercube);
                    let active = map.get(&(x, y, z, w)).is_some();
                    let new_active = match (active, count) {
                        (true, 2..=3) => true,
                        (false, 3) => true,
                        _ => false,
                    };
                    if new_active {
                        newmap.insert((x, y, z, w));
                    }
                }
            }
        }
    }

    newmap
}

fn main() {
    let contents = fs::read_to_string("day17.txt").expect("Something went wrong reading the file");

    let mut map = to_map(&contents);
    for _ in 0..6 {
        map = evolve(&map, false);
    }
    println!("Part 1: {}", map.len());

    map = to_map(&contents);
    for _ in 0..6 {
        map = evolve(&map, true);
    }
    println!("Part 2: {}", map.len());
}
