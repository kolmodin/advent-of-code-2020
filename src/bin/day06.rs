#![feature(iterator_fold_self)]

extern crate nom;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/day06.txt").expect("Something went wrong reading the file");

    let sum_anyone: usize = contents
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(|ln| ln.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum();
    println!("Part 1: {}", sum_anyone);

    let sum_everyone: usize = contents
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|ln| ln.chars().collect::<HashSet<_>>())
                .fold_first(|acc, ln| acc.intersection(&ln).cloned().collect())
                .unwrap()
                .len()
        })
        .sum();
    println!("Part 2: {}", sum_everyone);
}
