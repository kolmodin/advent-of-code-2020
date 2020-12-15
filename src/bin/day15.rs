use std::collections::HashMap;

fn solve(nums: &[i32], requested_index: usize) -> i32 {
    let mut hm: HashMap<i32, usize> = HashMap::new();

    for (pos, n) in nums.iter().enumerate() {
        hm.insert(*n, pos + 1);
    }

    let mut prev = 0;

    for index in nums.len() + 1..=requested_index - 1 {
        let next = match hm.get(&prev) {
            Some(prev_index) => (index - prev_index) as i32,
            None => 0,
        };
        hm.insert(prev, index);
        prev = next;
    }
    return prev;
}

fn main() {
    let nums: Vec<i32> = vec![0, 13, 1, 16, 6, 17];

    println!("Part 1: {}", solve(&nums, 2020));
    println!("Part 2: {}", solve(&nums, 30000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2020() {
        assert_eq!(solve(&vec![0, 3, 6], 2020), 436);
        assert_eq!(solve(&vec![1, 3, 2], 2020), 1);
        assert_eq!(solve(&vec![2, 1, 3], 2020), 10);
    }
}
