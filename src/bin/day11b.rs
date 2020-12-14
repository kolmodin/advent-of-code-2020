#![feature(iter_map_while)]

use std::convert::TryFrom;
use std::fs;
use std::iter::successors;
use std::str::from_utf8;

type Pos = (i32, i32);

#[derive(PartialEq, Eq)]
struct Map {
    vec: Vec<Vec<u8>>,
}

impl Map {
    fn new(map: Vec<Vec<u8>>) -> Map {
        Map { vec: map }
    }

    fn new_with_size(map: &Map, cell: u8) -> Map {
        Map {
            vec: map
                .vec
                .iter()
                .map(|row| row.iter().map(|_| cell).collect())
                .collect(),
        }
    }

    fn get(&self, pos: (i32, i32)) -> Option<u8> {
        let (ix, iy) = pos;
        let x = usize::try_from(ix).ok()?;
        let y = usize::try_from(iy).ok()?;
        self.vec.get(y).map(|row| row.get(x)).flatten().cloned()
    }

    fn set(&mut self, pos: (i32, i32), cell: u8) {
        let row = self.vec.get_mut(pos.1 as usize).unwrap();
        row[pos.0 as usize] = cell;
    }

    fn repeat_delta_from_start(
        &self,
        start: (i32, i32),
        delta: (i32, i32),
    ) -> impl Iterator<Item = (Pos, u8)> + '_ {
        successors(Some(start), move |d| Some(step(d, &delta)))
            .map_while(move |pos| self.get(pos).map(|cell| (pos, cell))).fuse()
    }

    fn print(&self) {
        for row in &self.vec {
            println!("{}", from_utf8(row).unwrap());
        }
    }

    fn iter(&self) -> impl Iterator<Item = (Pos, u8)> + '_ {
        self.iter_pos().zip(self.iter_cells())
    }

    fn iter_pos(&self) -> impl Iterator<Item = Pos> + '_ {
        (0..self.vec.len())
            .flat_map(move |row| (0..self.vec[0].len()).map(move |col| (col as i32, row as i32)))
    }

    fn iter_cells(&self) -> impl Iterator<Item = u8> + '_ {
        self.vec.iter().flat_map(|row| row.iter()).cloned()
    }
}

fn evolve(mmap: &Map) -> Map {
    let mut new_mmap = Map::new_with_size(&mmap, b'@');

    for (pos, this_cell) in mmap.iter() {
        if this_cell == b'.' {
            new_mmap.set(pos, b'.');
            continue;
        }
        let is_occupied = mmap.get(pos) == Some(b'#');

        let dirs: Vec<(i32, i32)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut adjacent_occupied = 0;

        for dir in dirs {
            for (_, cell) in mmap.repeat_delta_from_start(pos, dir).skip(1) {
                match cell {
                    b'L' => {
                        break;
                    }
                    b'#' => {
                        adjacent_occupied += 1;
                        break;
                    }
                    _ => {
                        // don't break, continue searching.
                    }
                }
            }
        }

        new_mmap.set(
            pos,
            match (is_occupied, adjacent_occupied) {
                (false, 0) => b'#',
                (true, n) if n >= 5 => b'L',
                _ => this_cell,
            },
        );
    }
    new_mmap
}

fn step(xy: &(i32, i32), dir: &(i32, i32)) -> (i32, i32) {
    (xy.0 + dir.0, xy.1 + dir.1)
}

fn main() {
    let contents = fs::read_to_string("day11.txt").expect("Something went wrong reading the file");

    let map: Vec<Vec<u8>> = contents.lines().map(|ln| ln.as_bytes().to_vec()).collect();

    let mut series = successors(Some(Map::new(map)), |m| Some(evolve(m)));

    let mut previous = series.next().unwrap();
    println!("first!");
    previous.print();

    for next in series {
        println!("step\n-----");
        next.print();
        if previous != next {
            previous = next;
            continue;
        }

        let all = next.iter_cells().filter(|cell| *cell == b'#').count();
        println!("occupied seats {}", all);
        break;
    }
}
