extern crate nom;
use nom::{
    character::complete::alpha1, character::complete::digit1, combinator::eof, combinator::map_opt,
    sequence::pair, sequence::terminated, IResult,
};
use std::collections::HashSet;
use std::fs;

/*
byr (Birth Year) - four digits; at least 1920 and at most 2002.
iyr (Issue Year) - four digits; at least 2010 and at most 2020.
eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
hgt (Height) - a number followed by either cm or in:
    If cm, the number must be at least 150 and at most 193.
    If in, the number must be at least 59 and at most 76.
hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
pid (Passport ID) - a nine-digit number, including leading zeroes.
cid (Country ID) - ignored, missing or not.
*/
fn is_valid(input: &str) -> bool {
    let key = &input[0..3];
    let value = &input[4..];

    let ok = match key {
        "byr" => {
            let year = &input[4..].parse::<i32>().unwrap();
            (1920..=2002).contains(year)
        }
        "iyr" => {
            let year = &input[4..].parse::<i32>().unwrap();
            (2010..=2020).contains(year)
        }
        "eyr" => {
            let year = &input[4..].parse::<i32>().unwrap();
            (2020..=2030).contains(year)
        }
        "hgt" => {
            let result: IResult<_, _> = pair(
                map_opt(digit1, |n: &str| n.parse::<i32>().ok()),
                terminated(alpha1, eof),
            )(value);
            match result {
                Ok((_, (num, "cm"))) => (150..=193).contains(&num),
                Ok((_, (num, "in"))) => (59..=76).contains(&num),
                _ => false,
            }
        }
        "hcl" => {
            value.len() == 7
                && value.starts_with('#')
                && value[1..].chars().all(|c| c.is_ascii_hexdigit())
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
        "pid" => value.len() == 9 && value.chars().all(|c| c.is_numeric()),
        _ => false,
    };

    // println!("checking {}: {}  ({})", key, ok, input);
    ok
}

fn main() {
    let contents = fs::read_to_string("day04.txt").expect("Something went wrong reading the file");

    let all_fields: HashSet<_> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let mut valid_passports = 0;
    for passport in contents.split("\n\n") {
        println!("---");
        println!("One passport: {}", &passport);
        let mut fields = Vec::<String>::new();
        for line in passport.lines() {
            for pair in line.split(' ') {
                if is_valid(&pair) {
                    fields.push(pair[0..3].to_string());
                }
            }
        }
        fields.push("cid".to_string());
        let found_fields: HashSet<_> = fields.iter().cloned().collect();
        if found_fields.is_superset(&all_fields) {
            valid_passports += 1;
            println!("this passport is valid");
        } else {
            println!("invalid passport!");
        }
    }

    println!(
        "{} passports, {} valid",
        contents.split("\n\n").count(),
        valid_passports
    );
}
