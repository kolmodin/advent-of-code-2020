extern crate nom;
use std::fs;

fn range(input: &str, low: i32, high: i32) -> i32 {
    let mut lo = low;
    let mut hi = high;
    for ch in input.chars().into_iter() {
        match ch {
            'F' | 'L' => {
                hi = lo + (hi - lo) / 2;
            }
            'B' | 'R' => {
                lo = lo + (hi - lo + 1) / 2;
            }
            _ => {}
        }
    }
    assert_eq!(lo, hi);
    lo
}

fn seat_id(input: &str) -> i32 {
    let row = range(&input[0..7], 0, 127);
    let col = range(&input[7..10], 0, 7);
    row * 8 + col
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day05.txt").expect("Something went wrong reading the file");

    let mut all_seats: Vec<i32> = contents.lines().map(seat_id).collect();
    all_seats.sort_unstable();
    let all_seats = all_seats;

    let max_seat_id = all_seats.iter().max().unwrap();

    println!("Part 1: {}", max_seat_id);

    let my_seat_id = all_seats
        .windows(2)
        .find_map(|w| {
            if w[0] != w[1] - 1 {
                Some(w[0] + 1)
            } else {
                None
            }
        })
        .unwrap();

    println!("Part 2: {}", my_seat_id);
}
