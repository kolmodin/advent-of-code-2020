use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn parse_input(inp: &str) -> Option<(Vec<usize>, Vec<usize>)> {
    inp.split("\n\n")
        .map(|player| {
            player
                .lines()
                .skip(1)
                .map(|ln| ln.parse::<usize>().unwrap())
                .collect()
        })
        .collect_tuple()
}

fn score(deck: &[usize]) -> usize {
    deck.iter()
        .rev()
        .zip(1..)
        .map(|(a, b)| a * b)
        .sum::<usize>()
}

fn main() {
    let contents = fs::read_to_string("day22.txt").expect("Something went wrong reading the file");

    let (player1, player2) = parse_input(&contents).unwrap();
    let winner_part1 = play(player1, player2, false).get_winner_deck();

    let (player1, player2) = parse_input(&contents).unwrap();
    let winner_part2 = play(player1, player2, true).get_winner_deck();

    println!("Part 1: {}", score(&winner_part1));
    println!("Part 2: {}", score(&winner_part2));
}

enum Outcome {
    Player1Win(Vec<usize>),
    Player2Win(Vec<usize>),
}

impl Outcome {
    fn get_winner_deck(self) -> Vec<usize> {
        match self {
            Outcome::Player1Win(deck) => deck,
            Outcome::Player2Win(deck) => deck,
        }
    }
}

fn play(mut player1: Vec<usize>, mut player2: Vec<usize>, can_recurse: bool) -> Outcome {
    let mut states = HashSet::new();

    while !player1.is_empty() && !player2.is_empty() {
        let state: (Vec<usize>, Vec<usize>) = (player1.clone(), player2.clone());
        if states.get(&state).is_some() {
            return Outcome::Player1Win(player1);
        }
        states.insert(state);

        let p1 = player1[0];
        let p2 = player2[0];
        player1.remove(0);
        player2.remove(0);

        if !can_recurse || player1.len() < p1 || player2.len() < p2 {
            if p1 > p2 {
                player1.push(p1);
                player1.push(p2);
            } else {
                player2.push(p2);
                player2.push(p1);
            }
            continue;
        }

        match play(
            player1.iter().take(p1).cloned().collect(),
            player2.iter().take(p2).cloned().collect(),
            can_recurse,
        ) {
            Outcome::Player1Win(_) => {
                player1.push(p1);
                player1.push(p2);
            }
            Outcome::Player2Win(_) => {
                player2.push(p2);
                player2.push(p1);
            }
        }
    }

    if player1.is_empty() {
        return Outcome::Player2Win(player2);
    }
    return Outcome::Player1Win(player1);
}
