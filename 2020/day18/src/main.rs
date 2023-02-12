static INPUT: &str = include_str!("input.txt");

fn eval(input: &str) -> i64 {
    let mut tokens = input.bytes().filter(|b| !b.is_ascii_whitespace());

    fn eval_inner(tokens: &mut dyn Iterator<Item = u8>) -> i64 {
        let mut acc = if let Some(t) = tokens.next() {
            if t == b'(' {
                eval_inner(tokens)
            } else {
                assert!(t.is_ascii_digit());
                (t - b'0') as i64
            }
        } else {
            0
        };

        while let Some(t) = tokens.next() {
            match t {
                b')' => break,
                b'+' | b'*' => {
                    let v = match tokens.next() {
                        Some(b'(') => eval_inner(tokens),
                        Some(b) if b.is_ascii_digit() => (b - b'0') as i64,
                        _ => panic!("cannot parse"),
                    };
                    match t {
                        b'+' => acc += v,
                        b'*' => acc *= v,
                        _ => unreachable!(),
                    }
                }
                _ => panic!("cannot parse"),
            }
        }

        acc
    }

    let res = eval_inner(&mut tokens);
    assert!(tokens.next().is_none());
    res
}

fn eval_2(input: &str) -> i64 {
    struct Lexer(Vec<u8>);
    impl Lexer {
        fn next(&mut self) -> Option<u8> {
            self.0.pop()
        }
        fn peek(&mut self) -> Option<u8> {
            self.0.last().copied()
        }

        fn new(input: &str) -> Self {
            let mut tokens: Vec<u8> = input.bytes().filter(|b| !b.is_ascii_whitespace()).collect();
            tokens.reverse();
            Lexer(tokens)
        }
    }
    let mut lexer = Lexer::new(input);

    fn eval_inner(lexer: &mut Lexer) -> i64 {
        let mut acc = if let Some(t) = lexer.next() {
            if t == b'(' {
                eval_inner(lexer)
            } else {
                assert!(t.is_ascii_digit());
                (t - b'0') as i64
            }
        } else {
            0
        };

        while let Some(t) = lexer.next() {
            match t {
                b')' => break,
                b'+' | b'*' => {
                    let mut v = match lexer.next() {
                        Some(b'(') => eval_inner(lexer),
                        Some(b) if b.is_ascii_digit() => (b - b'0') as i64,
                        _ => panic!("cannot parse"),
                    };

                    if t == b'*' {
                        while let Some(b'+') = lexer.peek() {
                            // need to add the next value to v first
                            lexer.next(); // consume b'+'

                            let v2 = match lexer.next() {
                                Some(b'(') => eval_inner(lexer),
                                Some(b) if b.is_ascii_digit() => (b - b'0') as i64,
                                o => panic!("cannot parse: {o:?}"),
                            };
                            v += v2;
                        }
                    }

                    match t {
                        b'+' => acc += v,
                        b'*' => acc *= v,
                        _ => unreachable!(),
                    }
                }
                _ => panic!("cannot parse"),
            }
        }

        acc
    }

    let res = eval_inner(&mut lexer);
    assert!(lexer.next().is_none());
    res
}
fn part_1(input: &str) -> i64 {
    input.lines().map(eval).sum()
}
fn part_2(input: &str) -> i64 {
    input.lines().map(eval_2).sum()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{eval, eval_2};

    #[test_case("1 + 2 * 3 + 4 * 5 + 6", 71)]
    #[test_case("1 + (2 * 3) + (4 * (5 + 6))", 51)]
    #[test_case("2 * 3 + (4 * 5)", 26)]
    #[test_case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437)]
    #[test_case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240)]
    #[test_case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632)]
    fn test_eval(input: &str, res: i64) {
        assert_eq!(res, eval(input));
    }

    #[test_case("1 + 2 * 3 + 4 * 5 + 6", 231)]
    #[test_case("1 + (2 * 3) + (4 * (5 + 6))", 51)]
    #[test_case("2 * 3 + (4 * 5)", 46)]
    #[test_case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445)]
    #[test_case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060)]
    #[test_case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340)]
    fn test_eval_2(input: &str, res: i64) {
        assert_eq!(res, eval_2(input));
    }
}
