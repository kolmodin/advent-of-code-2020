use std::fs;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Token {
    Eof,
    Num(i64),
    Op(char), // Left and right parens are stored here as operators, just as +*.
}

struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(mut inp: &str) -> Lexer {
        let mut tokens = Vec::new();
        while let Some(c) = inp.chars().next() {
            if c.is_ascii_whitespace() {
                inp = &inp[1..];
                continue;
            }
            if !c.is_digit(10) {
                tokens.push(Token::Op(c));
                inp = &inp[1..];
                continue;
            }

            let num: String = inp.chars().take_while(|c| c.is_digit(10)).collect();
            inp = &inp[num.len()..];
            tokens.push(Token::Num(num.parse::<i64>().unwrap()));
        }

        tokens.reverse();
        Lexer { tokens }
    }

    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    fn peek(&self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}

struct P<'a> {
    lexer: Lexer,
    ops: &'a [&'a str],
}

impl<'a> P<'a> {
    fn new(inp: &str, ops: &'a [&'a str]) -> P<'a> {
        P {
            lexer: Lexer::new(inp),
            ops,
        }
    }

    // digits
    // (expr)
    fn one(&mut self) -> Option<i64> {
        match self.lexer.peek() {
            Token::Op('(') => {
                self.lexer.next();
                let expr = self.expr(0);
                assert_eq!(self.lexer.next(), Token::Op(')'));
                expr
            }
            Token::Num(num) => {
                self.lexer.next();
                Some(num)
            }
            _ => None,
        }
    }

    // Parse operator at the expected level
    fn operator(&mut self, level: usize) -> Option<char> {
        match self.lexer.peek() {
            Token::Op(op) if self.ops[level].contains(op) => {
                self.lexer.next();
                Some(op)
            }
            _ => None,
        }
    }

    fn expr(&mut self, level: usize) -> Option<i64> {
        if level == self.ops.len() {
            // We reached the end of precedence climbing,
            // expect parens or digits with one().
            return self.one();
        }
        if let Token::Eof = self.lexer.peek() {
            return None;
        }

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
    let contents = fs::read_to_string("inputs/day18.txt").expect("Something went wrong reading the file");
    println!("Part 1: {}", contents.lines().map(eval_p1).sum::<i64>());
    println!("Part 2: {}", contents.lines().map(eval_p2).sum::<i64>());
}

fn eval_p1(inp: &str) -> i64 {
    let ops = vec!["+*"];
    let mut p = P::new(&inp, &ops);
    p.expr(0).unwrap()
}

fn eval_p2(inp: &str) -> i64 {
    let ops = vec!["*", "+"];
    let mut p = P::new(&inp, &ops);
    p.expr(0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day18_part1() {
        let contents =
            fs::read_to_string("inputs/day18.txt").expect("Something went wrong reading the file");
        assert_eq!(contents.lines().map(eval_p1).sum::<i64>(), 9535936849815);
    }
    #[test]
    fn test_day18_part2() {
        let contents =
            fs::read_to_string("inputs/day18.txt").expect("Something went wrong reading the file");
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
