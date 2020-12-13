use std::fs;

#[derive(Debug)]
struct Bus {
    id: usize,
    delay: usize,
}

fn parse_input(input: &str) -> (usize, Vec<Bus>) {
    let mut lns = input.lines();
    (
        lns.next().unwrap().parse().unwrap(),
        parse_busses(lns.next().unwrap()),
    )
}

fn parse_busses(input: &str) -> Vec<Bus> {
    input
        .split(',')
        .enumerate()
        .flat_map(|(delay, b)| match b {
            "x" => None,
            s => s.parse().ok().map(|id| Bus { id, delay }),
        })
        .collect()
}

fn solve_part1(my_arrival: usize, busses: &[Bus]) -> usize {
    let earliest_bus_departure = |bus: &Bus| ((my_arrival - 1) / bus.id + 1) * bus.id - my_arrival;
    busses
        .iter()
        .map(|bus| (bus.id, earliest_bus_departure(bus)))
        .min_by_key(|(_, wait)| *wait)
        .map(|(id, wait)| id * wait)
        .unwrap()
}

#[derive(Debug)]
struct Candidate {
    first: usize,
    mult: usize,
}

fn step(prev: Candidate, bus: &Bus) -> Candidate {
    let valid: Vec<_> = (prev.first..)
        .step_by(prev.mult)
        .filter(|t| (t + bus.delay) % bus.id == 0)
        .take(2)
        .collect();

    Candidate {
        first: valid[0],
        mult: valid[1] - valid[0],
    }
}

fn solve_part2(busses: &[Bus]) -> usize {
    busses
        .iter()
        .fold(Candidate { first: 1, mult: 1 }, step)
        .first
}

fn main() {
    let contents = fs::read_to_string("day13.txt").expect("Something went wrong reading the file");

    let (earliest_departure, busses) = parse_input(&contents);

    println!("part 1: {}", solve_part1(earliest_departure, &busses));

    println!("part 2: {}", solve_part2(&busses));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents =
            fs::read_to_string("day13.txt").expect("Something went wrong reading the file");
        let (earliest_departure, busses) = parse_input(&contents);

        assert_eq!(solve_part1(earliest_departure, &busses), 4938);
    }

    #[test]
    fn test_part2() {
        let contents =
            fs::read_to_string("day13.txt").expect("Something went wrong reading the file");
        let (_, busses) = parse_input(&contents);

        assert_eq!(solve_part2(&busses), 230903629977901);
    }
}
