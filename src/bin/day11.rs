#![feature(iter_map_while)]

use aoc2020::map::Map;
use aoc2020::pos2d::Pos;

use std::fs;
use std::iter::successors;

trait Rules {
    fn is_occupied_dir(&self, map: &Map, pos: Pos, dir: Pos) -> bool;
    fn new_cell(&self, is_occupied: bool, adjacent_occupied: usize) -> Option<u8>;
}

struct Part1;
impl Rules for Part1 {
    fn is_occupied_dir(&self, map: &Map, pos: Pos, dir: Pos) -> bool {
        map.get(pos + dir) == Some(b'#')
    }
    fn new_cell(&self, is_occupied: bool, adjacent_occupied: usize) -> Option<u8> {
        match (is_occupied, adjacent_occupied) {
            (false, 0) => Some(b'#'),
            (true, n) if n >= 4 => Some(b'L'),
            _ => None,
        }
    }
}

struct Part2;
impl Rules for Part2 {
    fn is_occupied_dir(&self, map: &Map, pos: Pos, dir: Pos) -> bool {
        map.repeat_delta_from_start(pos, dir)
            .skip(1)
            .find_map(|(_, cell)| match cell {
                b'L' => Some(false),

                b'#' => Some(true),
                _ => None,
            })
            .unwrap_or(false)
    }
    fn new_cell(&self, is_occupied: bool, adjacent_occupied: usize) -> Option<u8> {
        match (is_occupied, adjacent_occupied) {
            (false, 0) => Some(b'#'),
            (true, n) if n >= 5 => Some(b'L'),
            _ => None,
        }
    }
}

fn evolve<R: Rules>(mmap: &Map, rules: &R) -> Map {
    let mut new_mmap = Map::new_with_size(&mmap, b'@');

    for (pos, this_cell) in mmap.iter() {
        if this_cell == b'.' {
            new_mmap.set(pos, b'.');
            continue;
        }
        let is_occupied = this_cell == b'#';

        let adjacent_occupied = Pos::origo()
            .neighbors()
            .into_iter()
            .filter(|dir| rules.is_occupied_dir(&mmap, pos, *dir))
            .count();

        new_mmap.set(
            pos,
            rules
                .new_cell(is_occupied, adjacent_occupied)
                .unwrap_or(this_cell),
        );
    }
    new_mmap
}

fn run_evolve_with<R: Rules>(map: Map, rules: R) -> usize {
    let series = successors(Some(map), |prev| {
        let next = evolve::<R>(prev, &rules);
        if prev != &next {
            Some(next)
        } else {
            None
        }
    });

    let final_map = series.last().unwrap();

    final_map.iter_cells().filter(|cell| *cell == b'#').count()
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day11.txt").expect("Something went wrong reading the file");

    let map: Vec<Vec<u8>> = contents.lines().map(|ln| ln.as_bytes().to_vec()).collect();

    println!("Part 1: {}", run_evolve_with(Map::new(&map), Part1 {}));
    println!("Part 2: {}", run_evolve_with(Map::new(&map), Part2 {}));
}
