use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn parse_input(inp: &str) -> Option<(Vec<u8>, Vec<u8>)> {
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

fn score(deck: &[u8]) -> usize {
    deck.iter()
        .rev()
        .zip(1..)
        .map(|(&a, b)| a as usize * b)
        .sum()
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day22.txt").expect("Something went wrong reading the file");

    let (player1, player2) = parse_input(&contents).unwrap();
    let winner_part1 = play(&player1, &player2, false).get_deck();

    let (player1, player2) = parse_input(&contents).unwrap();
    let winner_part2 = play(&player1, &player2, true).get_deck();

    println!("Part 1: {}", score(&winner_part1));
    println!("Part 2: {}", score(&winner_part2));
}

enum Winner {
    Player1(Vec<u8>, usize),
    Player2(Vec<u8>, usize),
}

impl Winner {
    fn get_deck(self) -> Vec<u8> {
        match self {
            Winner::Player1(deck, front) => deck[front..].to_vec(),
            Winner::Player2(deck, front) => deck[front..].to_vec(),
        }
    }
}

fn deck_clone(s: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(1024);
    for i in s.iter() {
        v.push(*i);
    }
    v
}

fn play(source1: &[u8], source2: &[u8], can_recurse: bool) -> Winner {
    let mut player1 = deck_clone(source1);
    let mut player2 = deck_clone(source2);

    let mut states: HashMap<(usize, u8, usize, u8), Vec<(usize, usize, usize, usize)>> =
        HashMap::new();
    let mut front1 = 0;
    let mut front2 = 0;
    let is_empty = |arr: &[_], front| arr.len() == front;
    let len = |arr: &[_], front| arr.len() - front;
    let eq = |arr: &[u8], front1, len1, front2, len2| arr[front1..len1] == arr[front2..len2];

    while !is_empty(&player1, front1) && !is_empty(&player2, front2) {
        if can_recurse {
            let key: (usize, u8, usize, u8) = (
                len(&player1, front1),
                player1[front1],
                len(&player2, front2),
                player2[front2],
            );
            let value: (usize, usize, usize, usize) =
                (front1, player1.len(), front2, player2.len());
            let entry = states.entry(key).or_insert(vec![]);

            for (stored_front1, stored_len1, stored_front2, stored_len2) in entry.iter() {
                if eq(
                    &player1,
                    front1,
                    player1.len(),
                    *stored_front1,
                    *stored_len1,
                ) && eq(
                    &player2,
                    front2,
                    player2.len(),
                    *stored_front2,
                    *stored_len2,
                ) {
                    return Winner::Player1(player1, front1);
                }
            }

            entry.push(value);
        }

        let p1 = player1[front1];
        let p2 = player2[front2];
        front1 += 1;
        front2 += 1;

        if !can_recurse
            || len(&player1, front1) < p1 as usize
            || len(&player2, front2) < p2 as usize
        {
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
            &player1[front1..front1 + p1 as usize],
            &player2[front2..front2 + p2 as usize],
            can_recurse,
        ) {
            Winner::Player1(_, _) => {
                player1.push(p1);
                player1.push(p2);
            }
            Winner::Player2(_, _) => {
                player2.push(p2);
                player2.push(p1);
            }
        }
    }

    if front1 == player1.len() {
        return Winner::Player2(player2, front2);
    }
    Winner::Player1(player1, front1)
}
