use std::collections::HashSet;
use std::fs;

fn solve(nums: &[i32]) -> Option<(i32, i32, i32)> {
    let hm: HashSet<i32> = nums.iter().cloned().collect();
    for a in nums {
        for b in nums {
            let c = 2020 - a - b;
            if hm.get(&c).is_some() {
                return Some((*a, *b, c));
            }
        }
    }
    None
}

fn main() {
    let contents = fs::read_to_string("inputs/day01.txt").expect("Something went wrong reading the file");
    let nums: Vec<i32> = contents
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    if let Some((a, b, c)) = solve(&nums) {
        println!("{} * {} * {} = {}", a, b, c, a * b * c);
    } else {
        println!("could not find solution");
    }
}
