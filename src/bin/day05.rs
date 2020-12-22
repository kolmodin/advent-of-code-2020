extern crate nom;
use std::fs;

fn range(input: &str, low: i32, high: i32) -> i32 {
    // println!("{} {} {}", &input, low, high);
    let mut lo = low;
    let mut hi = high;
    // println!("Starting at lo {}, hi {}", lo, hi);
    for ch in input.chars().into_iter() {
        match ch {
            'F' | 'L' => {
                // low half
                hi = lo + (hi - lo) / 2;
                // println!("lower half lo {}, hi {}", lo, hi);
            }
            'B' | 'R' => {
                // high half
                lo = lo + (hi - lo + 1) / 2;
                // println!("upper half lo {}, hi {}", lo, hi);
            }
            _ => {}
        }
    }
    assert_eq!(lo, hi);
    lo
}

fn seat_id(input: &str) -> i32 {
    // println!("---\n{}", &input);
    let row = range(&input[0..7], 0, 127);
    let col = range(&input[7..10], 0, 7);
    // println!(
    //     "{}: row {}, column {}, seatID {}.",
    //     &input, row, col, seat_id
    // );
    row * 8 + col
}

fn main() {
    let contents = fs::read_to_string("inputs/day05.txt").expect("Something went wrong reading the file");

    // seat_id("FBFBBFFRLR");
    // seat_id("BFFFBBFRRR");
    // seat_id("FFFBBBFRRR");
    // seat_id("BBFFBBFRLL");
    let max = contents.lines().map(|ln| seat_id(ln)).max().unwrap();
    println!("maximum seat_id {}", max);

    let mut seat_ids = contents.lines().map(|ln| seat_id(ln)).collect::<Vec<_>>();
    seat_ids.sort_unstable();
    let mut start = seat_ids[0];
    for id in &seat_ids[1..] {
        if *id == start + 1 {
            start = *id;
        } else {
            println!("my id? {} {}", start, *id);
            start = *id;
        }
        println!("{}", id);
    }
}
