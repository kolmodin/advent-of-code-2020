use std::collections::HashMap;
use std::fs;

fn solve_part1(nums: &[i32]) -> usize {
        let diffs: Vec<_> = nums.windows(2).map(|w| w[1] - w[0]).collect();

        let ones = diffs.iter().filter(|i| **i == 1).count();
        let threes = diffs.iter().filter(|i| **i == 3).count();
        ones * threes
}

fn solve_part2(nums: &[i32], device_adapter: i32) -> i64 {
        let mut memory: HashMap<i32, i64> = HashMap::new();
        memory.insert(0, 1);

        for adapter in nums.iter().skip(1) {
                memory.insert(
                        *adapter,
                        (1..=3).filter_map(|i| memory.get(&(adapter - i))).sum(),
                );
        }

        *memory.get(&device_adapter).unwrap()
}

fn input_from_file(path: &str) -> (Vec<i32>, i32) {
        let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
        let mut nums: Vec<i32> = contents
                .lines()
                .map(|ln| ln.parse::<i32>().unwrap())
                .collect();
        let device_adapter = 3 + nums.iter().max().unwrap();

        nums.push(0);
        nums.push(device_adapter);
        nums.sort_unstable();

        (nums, device_adapter)
}

fn main() {
        let (nums, device_adapter) = input_from_file("inputs/day10.txt");
        println!("Part 1: {}", solve_part1(&nums));
        println!("Part 2: {}", solve_part2(&nums, device_adapter));
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn test_solve_part2_example() {
                let (nums, device_adapter) = input_from_file("inputs/day10_example.txt");

                assert_eq!(solve_part2(&nums, device_adapter), 19208);
        }

        #[test]
        fn test_solve_part2() {
                let (nums, device_adapter) = input_from_file("inputs/day10.txt");

                assert_eq!(solve_part2(&nums, device_adapter), 347250213298688);
        }
}
