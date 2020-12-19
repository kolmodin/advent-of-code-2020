fn solve(nums: &[usize], requested_index: usize) -> usize {
    let mut arr = vec![0; 1024];

    for (pos, n) in nums.iter().enumerate() {
        arr[*n] = pos + 1;
    }

    let mut prev = 0;

    for index in nums.len() + 1..=requested_index - 1 {
        while prev >= arr.len() {
            arr.resize(arr.len() * 2, 0);
        }
        let next = match arr[prev] {
            0 => 0,
            prev_index => index - prev_index,
        };
        arr[prev] = index;
        prev = next;
    }
    return prev;
}

fn main() {
    let nums: Vec<usize> = vec![0, 13, 1, 16, 6, 17];

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
