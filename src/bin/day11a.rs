use std::collections::HashMap;
use std::fs;
use std::iter::successors;
use std::str::from_utf8;

fn evolve(map: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut new_map: Vec<Vec<u8>> = Vec::new();
    let maphs: HashMap<(i32, i32), u8> = {
        let mut new = HashMap::<(i32, i32), u8>::new();
        for (y, row) in map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                new.insert((x as i32, y as i32), *c);
            }
        }
        new
    };

    for (y, row) in map.iter().enumerate() {
        new_map.push(vec![]);
        for (x, this_seat) in row.iter().enumerate() {
            if *this_seat == b'.' {
                new_map.last_mut().unwrap().push(b'.');
                continue;
            }
            let is_occupied = map[y][x] == b'#';

            let adjacent_seats: Vec<(i32, i32)> = vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];
            let adjacent_occupied: usize = adjacent_seats
                .iter()
                .filter_map(|(dx, dy)| maphs.get(&(x as i32 + dx, y as i32 + dy)))
                .filter(|c| **c == b'#')
                .count();

            new_map
                .last_mut()
                .unwrap()
                .push(match (is_occupied, adjacent_occupied) {
                    (false, 0) => b'#',
                    (true, n) if n >= 4 => b'L',
                    _ => *this_seat,
                });
        }
    }

    new_map
}

fn print_map(map: &Vec<Vec<u8>>) {
    println!("----");
    for row in map {
        println!("{}", from_utf8(row).unwrap());
    }
}

fn map_eq(map1: &Vec<Vec<u8>>, map2: &Vec<Vec<u8>>) -> bool {
    for (y, row) in map1.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if map2.get(y).map(|row2| row2.get(x)).flatten() != Some(c) {
                println!("{} {}", x,y);
                return false;
            }
        }
    }
    true
}

fn main() {
    let contents =
        fs::read_to_string("day11.txt").expect("Something went wrong reading the file");

    let map: Vec<Vec<u8>> = contents.lines().map(|ln| ln.as_bytes().to_vec()).collect();

    let mut series = successors(Some(map), |m| Some(evolve(m)));

    let mut previous = series.next().unwrap();
    println!("first!");
    print_map(&previous);

    for next in series {
        println!("step");
        print_map(&next);
        if !map_eq(&previous, &next) {
            println!("different");
            previous = next;
            continue;
        }

        let all = next
            .iter()
            .flat_map(|row| row.iter().filter(|seat| **seat == b'#'))
            .count();
        println!("success {}!", all);
        break;
    }
}
