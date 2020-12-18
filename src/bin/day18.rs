use std::fs;

struct P<'a> {
    inp: &'a str,
    ops: &'a [&'a str],
}

impl<'a> P<'a> {
    fn new(inp: &'a str, ops: &'a [&'a str]) -> P<'a> {
        P { inp, ops }
    }

    // digits
    // (expr)
    fn one(&mut self) -> Option<i64> {
        if self.inp.is_empty() {
            return None;
        };

        if self.inp.chars().next() == Some('(') {
            self.inp = &self.inp[1..];
            let val = self.expr(0);
            assert_eq!(self.inp.chars().next(), Some(')'));
            self.inp = &self.inp[1..];
            return val;
        };

        let mut count = 0;
        let mut num: Vec<char> = Vec::new();
        for c in self.inp.chars() {
            if c.is_digit(10) {
                num.push(c);
                count += 1;
            } else {
                break;
            }
        }
        assert_ne!(count, 0);
        self.inp = &self.inp[count..];
        let s: String = num.into_iter().collect();
        Some(s.parse::<i64>().unwrap())
    }

    // parse operator at the expected level
    fn operator(&mut self, level: usize) -> Option<char> {
        if self.inp.is_empty() {
            return None;
        };

        let op = self.inp.chars().next().unwrap();
        if !self.ops[level].contains(op) {
            return None;
        }

        self.inp = &self.inp[1..];

        return Some(op);
    }

    fn expr(&mut self, level: usize) -> Option<i64> {
        if level == self.ops.len() {
            // we reached the ende of precedence climbing,
            // expect parens or digits with one().
            return self.one();
        }
        if self.inp.is_empty() {
            return None;
        };
        let mut acc = self.expr(level + 1).unwrap();
        while let Some(op) = self.operator(level) {
            let next = self.expr(level + 1).unwrap();
            acc = match op {
                '+' => acc + next,
                '*' => acc * next,
                c => panic!("invalid operator {c}", c = c),
            };
        }
        Some(acc)
    }
}

fn main() {
    let contents = fs::read_to_string("day18.txt").expect("Something went wrong reading the file");
    println!("Part 1: {}", contents.lines().map(eval_p1).sum::<i64>());
    println!("Part 2: {}", contents.lines().map(eval_p2).sum::<i64>());
}

fn eval_p1(inp: &str) -> i64 {
    let s: String = inp.chars().filter(|c| *c != ' ').collect();
    let ops = vec!["+*"];
    let mut p = P::new(&s, &ops);
    p.expr(0).unwrap()
}

fn eval_p2(inp: &str) -> i64 {
    let s: String = inp.chars().filter(|c| *c != ' ').collect();
    let ops = vec!["*", "+"];
    let mut p = P::new(&s, &ops);
    p.expr(0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day18_part1() {
        let contents =
            fs::read_to_string("day18.txt").expect("Something went wrong reading the file");
        assert_eq!(contents.lines().map(eval_p1).sum::<i64>(), 9535936849815);
    }
    #[test]
    fn test_day18_part2() {
        let contents =
            fs::read_to_string("day18.txt").expect("Something went wrong reading the file");
        assert_eq!(contents.lines().map(eval_p2).sum::<i64>(), 472171581333710);
    }

    #[test]
    fn test_day18_ex1_p1() {
        assert_eq!(eval_p1("2 * 3 + (4 * 5)"), 26);
    }

    #[test]
    fn test_day18_ex2_p1() {
        assert_eq!(eval_p1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    }

    #[test]
    fn test_day18_ex3_p1() {
        assert_eq!(eval_p1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    }

    #[test]
    fn test_day18_ex4_p1() {
        assert_eq!(
            eval_p1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn test_day18_ex1_p2() {
        assert_eq!(eval_p2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    }

    #[test]
    fn test_day18_ex2_p2() {
        assert_eq!(eval_p2("2 * 3 + (4 * 5)"), 46);
    }

    #[test]
    fn test_day18_ex3_p2() {
        assert_eq!(eval_p2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
    }

    #[test]
    fn test_day18_ex4_p2() {
        assert_eq!(eval_p2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
    }

    #[test]
    fn test_day18_ex5_p2() {
        assert_eq!(
            eval_p2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}
