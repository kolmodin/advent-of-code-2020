use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn parse_input(inp: &str) -> Option<(Vec<usize>, Vec<usize>)> {
    inp.split("\n\n")
        .map(|player| {
            player
                .lines()
                .skip(1)
                .map(|ln| ln.parse().unwrap())
                .collect()
        })
        .collect_tuple()
}

fn score(deck: &[usize]) -> usize {
    deck.iter()
        .rev()
        .zip(1..)
        .map(|(a, b)| a * b)
        .sum()
}

fn main() {
    let contents = fs::read_to_string("inputs/day22.txt").expect("Something went wrong reading the file");

    let (player1, player2) = parse_input(&contents).unwrap();
    let winner_part1 = play(player1, player2, false).get_deck();

    let (player1, player2) = parse_input(&contents).unwrap();
    let winner_part2 = play(player1, player2, true).get_deck();

    println!("Part 1: {}", score(&winner_part1));
    println!("Part 2: {}", score(&winner_part2));
}

enum Winner {
    Player1(Vec<usize>),
    Player2(Vec<usize>),
}

impl Winner {
    fn get_deck(self) -> Vec<usize> {
        match self {
            Winner::Player1(deck) => deck,
            Winner::Player2(deck) => deck,
        }
    }
}

fn play(mut player1: Vec<usize>, mut player2: Vec<usize>, can_recurse: bool) -> Winner {
    let mut states = HashSet::new();

    while !player1.is_empty() && !player2.is_empty() {
        let state: (Vec<usize>, Vec<usize>) = (player1.clone(), player2.clone());
        if states.get(&state).is_some() {
            return Winner::Player1(player1);
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
            Winner::Player1(_) => {
                player1.push(p1);
                player1.push(p2);
            }
            Winner::Player2(_) => {
                player2.push(p2);
                player2.push(p1);
            }
        }
    }

    if player1.is_empty() {
        return Winner::Player2(player2);
    }
    Winner::Player1(player1)
}
