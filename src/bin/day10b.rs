use std::collections::HashMap;
use std::fs;

fn solve(mut nums: Vec<i32>) -> Option<i64> {
    let device_adapter = 3 + nums.iter().max().unwrap();

    nums.push(0);
    nums.push(device_adapter);
    nums.sort_unstable();

    let mut memory: HashMap<i32, i64> = HashMap::new();
    memory.insert(0, 1);

    for adapter in nums.iter().skip(1) {
        memory.insert(
            *adapter,
            (1..=3).filter_map(|i| memory.get(&(adapter - i))).sum(),
        );
    }

    memory.get(&device_adapter).copied()
}

fn parse_input(path: &str) -> Vec<i32> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents.lines().map(|ln| ln.parse().unwrap()).collect()
}

fn main() {
    let nums = parse_input("inputs/day10.txt");

    println!("device adapter {}", solve(nums).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_solve_example() {
        let nums = parse_input("day10_example.txt");

        assert_eq!(solve(nums), Some(19208));
    }

    #[test]
    #[ignore]
    fn test_solve() {
        let nums = parse_input("day10.txt");

        assert_eq!(solve(nums), Some(347250213298688));
    }
}
