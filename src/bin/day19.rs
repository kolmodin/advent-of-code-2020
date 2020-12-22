use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

type ID = usize;
type Start = usize;
type End = usize;

type Memory = HashMap<(ID, Start), Vec<End>>;

#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
enum Rule {
    Exact(u8),
    Or(Vec<Vec<ID>>),
}

fn parse_rule(inp: &str) -> (ID, Rule) {
    // 131: 134 72 | 95 5
    // 72: "b"
    let parts: Vec<&str> = inp.split(": ").collect();
    let num = parts[0].parse::<ID>().unwrap();
    if parts[1].starts_with('"') {
        return (num, Rule::Exact(parts[1].as_bytes()[1]));
    }
    let sets: Vec<&str> = parts[1].split(" | ").collect();
    return (
        num,
        Rule::Or(
            sets.iter()
                .map(|set| {
                    set.split(' ')
                        .map(|i| i.parse::<ID>().unwrap())
                        .collect::<Vec<ID>>()
                })
                .collect(),
        ),
    );
}

fn validate_msg(rules: &[Rule], inp: &[u8]) -> bool {
    run_rule(&mut HashMap::new(), rules, inp, 0, 0).contains(&inp.len())
}

fn run_rule(memory: &mut Memory, rules: &[Rule], inp: &[u8], id: ID, start: Start) -> Vec<End> {
    if let Some(ends) = memory.get(&(id, start)) {
        return ends.clone();
    }
    if start >= inp.len() {
        memory.insert((id, start), vec![]);
        return vec![];
    }
    let ends: Vec<End> = match rules.get(id).unwrap() {
        Rule::Exact(c) => {
            if inp[start] == *c {
                vec![start + 1]
            } else {
                vec![]
            }
        }
        Rule::Or(ors) => ors
            .iter()
            // Run the ORed rules separately and flatten the result.
            .flat_map(|ands| {
                // Run the sequence of chained rules with fold.
                ands.iter().fold(vec![start], |starts, and_id| {
                    // Produce end states for all start positions for the given rule.
                    starts
                        .iter()
                        .flat_map(|start| run_rule(memory, rules, inp, *and_id, *start))
                        .collect()
                })
            })
            .dedup()
            .collect(),
    };

    memory.insert((id, start), ends.clone());
    ends
}

fn update_rules_for_part2(mut rules: Vec<Rule>) -> Vec<Rule> {
    for (id, rule) in vec![
        "8: 42 | 42 8", // .
        "11: 42 31 | 42 11 31",
    ]
    .into_iter()
    .map(parse_rule)
    {
        rules[id] = rule;
    }
    rules
}

fn main() {
    let contents = fs::read_to_string("inputs/day19.txt").expect("Something went wrong reading the file");
    let parts: Vec<&str> = contents.split("\n\n").collect();

    let mut rules: Vec<(ID, Rule)> = parts[0].lines().map(parse_rule).collect();
    rules.sort();

    let rules: Vec<Rule> = rules.into_iter().map(|(_, r)| r).collect();
    let messages: Vec<&[u8]> = parts[1].lines().map(|ln| ln.as_bytes()).collect();

    println!(
        "Part 1 (of {} messages): {}",
        messages.len(),
        messages
            .iter()
            .filter(|msg| validate_msg(&rules, msg))
            .count()
    );

    let rules = update_rules_for_part2(rules);

    println!(
        "Part 2 (of {} messages): {}",
        messages.len(),
        messages
            .iter()
            .filter(|msg| validate_msg(&rules, msg))
            .count()
    );
}
