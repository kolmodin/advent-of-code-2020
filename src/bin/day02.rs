extern crate nom;
use nom::{
    character::complete::alpha1,
    character::complete::anychar,
    character::complete::char,
    character::complete::digit1,
    bytes::complete::tag,
    combinator::eof,
    sequence::preceded,
    sequence::tuple,
    Finish,
    IResult,
};

use std::fs;

#[derive(Debug)]
struct Line {
    min: usize,
    max: usize,
    c: char,
    pw: String,
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    // 12-14 s: ssskrssssssfsxpsqsp
    let (input, (min_str, max_str, c, pw)) = tuple((
        digit1,
        preceded(char('-'), digit1),
        preceded(char(' '), anychar),
        preceded(tag(": "), alpha1),
    ))(input)?;

    let (input, _) = eof(input)?;

    Ok((
        input,
        Line {
            min: min_str.parse::<usize>().unwrap(),
            max: max_str.parse::<usize>().unwrap(),
            c,
            pw: String::from(pw),
        },
    ))
}

fn is_valid1(line: &Line) -> bool {
    let num = line.pw.chars().filter(|c| *c == line.c).count();
    num >= line.min && num <= line.max
}

fn is_valid2(line: &Line) -> bool {
    let s = line.pw.as_bytes();
    let match1 = s[line.min - 1] == line.c as u8;
    let match2 = s[line.max - 1] == line.c as u8;
    (match1 && !match2) || (match2 && !match1)
}

fn main() {
    let contents = fs::read_to_string("day02.txt").expect("Something went wrong reading the file");

    let lines: Vec<Line> = contents
        .lines()
        .map(|s| parse_line(s).finish().map(|t| t.1).unwrap())
        .collect();

        let valid1 = lines.iter().filter(|ln| is_valid1(ln)).count();
        let valid2 = lines.iter().filter(|ln| is_valid2(ln)).count();

    println!("{} passwords, {} valid in part 1", lines.len(), valid1);
    println!("{} passwords, {} valid in part 2", lines.len(), valid2);
}
