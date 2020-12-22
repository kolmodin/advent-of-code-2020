use std::fs;

fn main() {

        let contents = fs::read_to_string("inputs/day10.txt").expect("Something went wrong reading the file");

        let mut nums: Vec<i32> = contents.lines().map(|ln| ln.parse::<i32>().unwrap()).collect();

        let device_adapter = 3 + nums.iter().max().unwrap();

        nums.push(0);
        nums.push(device_adapter);
        nums.sort_unstable();

        let diffs: Vec<i32> = nums.iter().skip(1).zip(&nums).map(|(a,b)| a-b).collect();

        let ones = diffs.iter().filter(|i| **i == 1).count();
        let threes = diffs.iter().filter(|i| **i == 3).count();
        
        println!("{:#?}", diffs);
        println!("1s {} * 3s {} = {}", ones, threes, ones*threes);
        
}
