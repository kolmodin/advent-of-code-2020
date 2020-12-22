use std::fs;
use std::iter::successors;

fn run_slope(map: &[&[u8]], right: usize, down: usize) -> usize {
    let width = map[0].len();
    let rows = map.iter().step_by(down);
    let cols = successors(Some(0_usize), |col| Some((col + right) % width));
    rows.zip(cols)
        .filter(|(row, col)| row[*col] == b'#')
        .count() as usize
}

fn main() {
    let contents = fs::read_to_string("inputs/day03.txt").expect("Something went wrong reading the file");

    let lines: Vec<&[u8]> = contents.lines().map(|ln| ln.as_bytes()).collect();

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let product: usize = slopes
        .iter()
        .map(|(right, down)| {
            let trees = run_slope(&lines, *right, *down);
            println!(
                "{} lines, right {}, down {}, hit {} trees",
                lines.len(),
                right,
                down,
                trees
            );
            trees
        })
        .product();

    println!("Product of trees: {}", product);
}
