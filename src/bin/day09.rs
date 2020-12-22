use std::fs;

fn solve(nums: &[i64], step: usize) -> Option<i64> {
    let mut start = 0;
    while start + step < nums.len() {
        let slice = &nums[start..start + step];
        let last = nums[start + step];
        let mut found = false;
        for a in slice {
            for b in slice {
                if a == b {
                    continue;
                }
                if last == a + b {
                    found = true;
                }
            }
        }
        if !found {
            return Some(last);
        }
        start += 1;
    }
    None
}

fn solve2(nums: &[i64], target: i64) -> Option<i64> {
    for start in 0..nums.len() {
        for end in start+1..nums.len() {
            let slice = &nums[start..end];
            let sum = slice.iter().sum::<i64>();
            if sum > target {
                break;
            }
            if sum == target {
                let min = slice.iter().min().unwrap();
                let max = slice.iter().max().unwrap();
                return Some(min + max);
            }
        }
    }
None
}

fn main() {
    let contents = fs::read_to_string("inputs/day09.txt").expect("Something went wrong reading the file");
    let nums: Vec<i64> = contents
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    if let Some(target) = solve(&nums, 25) {
        println!("{:?}", target);
        println!("{:?}", solve2(&nums, target));
    }

    println!("done");
}
