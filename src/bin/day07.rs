#![feature(str_split_once)]

extern crate nom;
use std::collections::HashMap;
use std::fs;

struct BagCount<'a> {
    name: &'a str,
    count: i32,
}

fn parse_line(input: &str) -> (&str, Vec<BagCount>) {
    let split0 = input
        .trim_end_matches('.')
        .split(" bags contain ")
        .collect::<Vec<_>>();
    let this_bag = split0[0];
    if split0[1] == "no other bags" {
        return (this_bag, vec![]);
    }
    let contains = split0[1]
        .split(", ")
        .map(|bag| {
            let parts = bag.split_once(" ").unwrap();
            let name = parts.1.split(" bag").next().unwrap();
            BagCount {
                count: parts.0.parse::<i32>().unwrap(),
                name,
            }
        })
        .collect();
    (this_bag, contains)
}

fn can_hold_my_bag<'a>(
    some_bag: &'a str,
    my_bag: &'a str,
    memory: &mut HashMap<&'a str, bool>,
    all_bags: &HashMap<&'a str, Vec<BagCount<'a>>>,
) -> bool {
    if *memory.get(some_bag).unwrap_or(&false) {
        return true;
    }
    let contents = all_bags.get(some_bag).unwrap();
    for bag_count in contents {
        if bag_count.name == my_bag {
            memory.insert(some_bag, true);
            return true;
        }
        if can_hold_my_bag(bag_count.name, my_bag, memory, all_bags) {
            memory.insert(some_bag, true);
            return true;
        }
    }
    memory.insert(some_bag, false);
    false
}

fn bag_counter<'a>(
    some_bag: &'a str,
    memory: &mut HashMap<&'a str, i32>,
    all_bags: &HashMap<&'a str, Vec<BagCount<'a>>>,
) -> i32 {
    if let Some(prev) = memory.get(some_bag) {
        return *prev;
    }
    let counter = 1 + all_bags
        .get(some_bag)
        .unwrap()
        .iter()
        .map(|bag_count| bag_count.count * bag_counter(bag_count.name, memory, all_bags))
        .sum::<i32>();
    memory.insert(some_bag, counter);
    counter
}

fn main() {
    let contents = fs::read_to_string("inputs/day07.txt").expect("Something went wrong reading the file");

    let bags: HashMap<&str,Vec<BagCount>> = contents.lines().map(|ln| parse_line(ln)).collect();

    // for bag in &bags {
    //     println!("{} has bags:", bag.0);
    //     for b in bag.1 {
    //         println!(" - {} {}", b.count, b.name);
    //     }
    // }
    let mut can_contain_shiny = HashMap::new();
    for name in bags.keys() {
        can_hold_my_bag(name, "shiny gold", &mut can_contain_shiny, &bags);
    }
    println!(
        "{} bags can contain my shiny gold bag",
        can_contain_shiny.values().filter(|b| **b).count()
    );
    let mut bag_contents = HashMap::new();
    println!(
        "{} bags inside shiny gold bag",
        bag_counter("shiny gold", &mut bag_contents, &bags) - 1
    );
}
