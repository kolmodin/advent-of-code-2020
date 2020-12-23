fn step(circle: &mut [usize], current_cup: usize) {
    let pickup_start = circle[current_cup];
    let pickup_end = circle[circle[pickup_start]];
    let pickup_end_next = circle[pickup_end];

    let picked_up_elems = [
        circle[current_cup],
        circle[circle[current_cup]],
        circle[circle[circle[current_cup]]],
    ];

    circle[current_cup] = pickup_end_next;

    let dest = {
        let mut dest = current_cup - 1;
        while picked_up_elems.contains(&dest) || dest == 0 {
            if dest == 0 {
                dest = circle.len() - 1;
            } else {
                dest -= 1 % (circle.len() - 1);
            }
        }
        dest
    };

    let dest_next = circle[dest];
    circle[dest] = pickup_start;
    circle[pickup_end] = dest_next;
}

fn normalize(circle: &[usize]) -> String {
    let mut nums= Vec::new();

    let count = circle.len() - 2;

    let mut next = circle[1];
    for _ in 0..count {
        nums.push(next);
        next = circle[next];
    }
    let strs: Vec<String> = nums.iter().map(|n| format!("{}", *n)).collect();
    strs.join("")
}

fn create_circle(nums: &[usize]) -> Vec<usize> {
    let mut circle: Vec<usize> = Vec::new();
    circle.resize(nums.len() + 1, 0);
    circle[0] = nums[0];

    for w in nums.windows(2) {
        circle[w[0]] = w[1];
    }
    circle[nums[nums.len() - 1]] = nums[0];

    circle
}

fn main() {
    let nums = vec![5, 8, 6, 4, 3, 9, 1, 7, 2];

    let mut circle = create_circle(&nums);

    let mut current_cup = circle[0];
    for _ in 0..100 {
        step(&mut circle, current_cup);
        current_cup = circle[current_cup];
    }

    println!("Part 1: {}", normalize(&circle));

    let nums = {
        let mut nums = nums;
        nums.extend(nums.len() + 1..=1_000_000);
        nums
    };

    let mut circle = create_circle(&nums);

    let mut current_cup = circle[0];
    for _ in 0..10_000_000 {
        step(&mut circle, current_cup);
        current_cup = circle[current_cup];
    }

    let a = circle[1];
    let b = circle[a];
    println!("Part 2: {} * {} = {}", a, b, a * b);
}
