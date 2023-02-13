use std::collections::HashMap;

static INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
enum RuleType {
    MatchByte(u8),
    Sequence(Vec<usize>),
    Alt(Vec<usize>, Vec<usize>),
}

#[derive(Debug, Clone)]
struct Rule {
    id: usize,
    rule: RuleType,
}

impl Rule {
    fn parse_str(input: &str) -> Self {
        let parts: Vec<_> = input.split(": ").collect();
        let id = parts[0].parse::<usize>().unwrap();
        let rule = if parts[1].as_bytes()[0] == b'"' {
            RuleType::MatchByte(parts[1].as_bytes()[1])
        } else {
            let alts: Vec<Vec<usize>> = parts[1]
                .split(" | ")
                .map(|s| {
                    s.split_whitespace()
                        .map(|i| i.parse::<usize>().unwrap())
                        .collect()
                })
                .collect();
            if alts.len() == 1 {
                RuleType::Sequence(alts[0].clone())
            } else {
                RuleType::Alt(alts[0].clone(), alts[1].clone())
            }
        };
        Self { id, rule }
    }
}

#[derive(Debug)]
struct Rules {
    rules: HashMap<usize, Rule>,
}

impl Rules {
    fn parse_str(input: &str) -> Self {
        let mut rules = HashMap::new();

        for line in input.lines() {
            let rule = Rule::parse_str(line);
            rules.insert(rule.id, rule);
        }

        Self { rules }
    }

    fn patch(mut self) -> Self {
        self.rules.insert(
            8,
            Rule {
                id: 8,
                rule: RuleType::Alt(vec![42], vec![42, 8]),
            },
        );
        self.rules.insert(
            11,
            Rule {
                id: 11,
                rule: RuleType::Alt(vec![42, 31], vec![42, 11, 31]),
            },
        );
        self
    }

    fn match_str(&self, input: &str) -> bool {
        let input = input.as_bytes();

        // DPS - (input to match, cur_rule to match, rest of rules in rev order (for easy pop))
        let mut queue = Vec::new();
        queue.push((input, 0, vec![]));

        while let Some((input, cur_rule, mut other_rules)) = queue.pop() {
            let Some(rule) = self.rules.get(&cur_rule) else { panic!("Unknown rule({cur_rule})")};
            match rule.rule {
                RuleType::MatchByte(b) => {
                    if input.first() == Some(&b) {
                        if let Some(cur_rule) = other_rules.pop() {
                            queue.push((&input[1..], cur_rule, other_rules));
                        } else if input.len() == 1 {
                            return true;
                        }
                    }
                }
                RuleType::Sequence(ref s) => {
                    other_rules.extend(s.iter().cloned().rev());
                    let Some(cur_rule) = other_rules.pop() else { panic!("Empty sequence for rule {cur_rule}")};
                    queue.push((input, cur_rule, other_rules));
                }
                RuleType::Alt(ref s1, ref s2) => {
                    let mut other_rules_2 = other_rules.clone();

                    other_rules.extend(s1.iter().cloned().rev());
                    let Some(cur_rule) = other_rules.pop() else { panic!("Empty sequence for alt 1 in rule {cur_rule}")};
                    queue.push((input, cur_rule, other_rules));

                    other_rules_2.extend(s2.iter().cloned().rev());
                    let Some(cur_rule) = other_rules_2.pop() else { panic!("Empty sequence for alt 2 in rule {cur_rule}")};
                    queue.push((input, cur_rule, other_rules_2));
                }
            }
        }

        false
    }
}

struct Input<'a> {
    rules: Rules,
    messages: Vec<&'a str>,
}

impl<'a> Input<'a> {
    fn parse_str(input: &'a str) -> Self {
        let parts: Vec<_> = input.split("\n\n").collect();
        let rules = Rules::parse_str(parts[0]);
        let messages = parts[1].lines().collect();
        Input { rules, messages }
    }

    fn patch(self) -> Self {
        Self {
            rules: self.rules.patch(),
            ..self
        }
    }
}

fn part_1(input: &str) -> usize {
    let input = Input::parse_str(input);

    input
        .messages
        .iter()
        .filter(|m| input.rules.match_str(m))
        .count()
}

fn part_2(input: &str) -> usize {
    let input = Input::parse_str(input).patch();

    input
        .messages
        .iter()
        .filter(|m| input.rules.match_str(m))
        .count()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{part_1, part_2, Rules, INPUT};

    static SMALL_INPUT: &str = r#"0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b""#;

    static LARGER_INPUT: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b""#;

    static TEST_INPUT: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    #[test_case(SMALL_INPUT, "aab", true)]
    #[test_case(SMALL_INPUT, "aba", true)]
    #[test_case(SMALL_INPUT, "abaa", false)]
    #[test_case(SMALL_INPUT, "ab", false)]
    #[test_case(SMALL_INPUT, "abb", false)]
    #[test_case(LARGER_INPUT, "aaaabb", true)]
    #[test_case(LARGER_INPUT, "aaabab", true)]
    #[test_case(LARGER_INPUT, "abbabb", true)]
    #[test_case(LARGER_INPUT, "abbbab", true)]
    #[test_case(LARGER_INPUT, "aabaab", true)]
    #[test_case(LARGER_INPUT, "aabbbb", true)]
    #[test_case(LARGER_INPUT, "abaaab", true)]
    #[test_case(LARGER_INPUT, "ababbb", true)]
    fn test_match_str_no_cycle(rules: &str, input: &str, m: bool) {
        let rules = Rules::parse_str(rules);
        assert_eq!(m, rules.match_str(input));
    }

    #[test_case(SMALL_INPUT, "aab", true)]
    #[test_case(SMALL_INPUT, "aba", true)]
    #[test_case(SMALL_INPUT, "abaa", false)]
    #[test_case(SMALL_INPUT, "ab", false)]
    #[test_case(SMALL_INPUT, "abb", false)]
    #[test_case(LARGER_INPUT, "aaaabb", true)]
    #[test_case(LARGER_INPUT, "aaabab", true)]
    #[test_case(LARGER_INPUT, "abbabb", true)]
    #[test_case(LARGER_INPUT, "abbbab", true)]
    #[test_case(LARGER_INPUT, "aabaab", true)]
    #[test_case(LARGER_INPUT, "aabbbb", true)]
    #[test_case(LARGER_INPUT, "abaaab", true)]
    #[test_case(LARGER_INPUT, "ababbb", true)]
    fn test_match_str(rules: &str, input: &str, m: bool) {
        let rules = Rules::parse_str(rules);
        assert_eq!(m, rules.match_str(input));
    }

    #[test_case(TEST_INPUT, 2)]
    fn test_part_1(input: &str, c: usize) {
        assert_eq!(c, part_1(input));
    }

    #[test_case(
        r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#,
        12
    )]
    #[test_case(INPUT, 282)]
    fn test_part_2(input: &str, c: usize) {
        assert_eq!(c, part_2(input));
    }

    static PART_2_RULES: &str = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1"#;

    #[test_case(PART_2_RULES, "bbabbbbaabaabba")]
    #[test_case(PART_2_RULES, "babbbbaabbbbbabbbbbbaabaaabaaa")]
    #[test_case(PART_2_RULES, "aaabbbbbbaaaabaababaabababbabaaabbababababaaa")]
    #[test_case(PART_2_RULES, "bbbbbbbaaaabbbbaaabbabaaa")]
    #[test_case(PART_2_RULES, "bbbababbbbaaaaaaaabbababaaababaabab")]
    #[test_case(PART_2_RULES, "ababaaaaaabaaab")]
    #[test_case(PART_2_RULES, "ababaaaaabbbaba")]
    #[test_case(PART_2_RULES, "baabbaaaabbaaaababbaababb")]
    #[test_case(PART_2_RULES, "abbbbabbbbaaaababbbbbbaaaababb")]
    #[test_case(PART_2_RULES, "aaaaabbaabaaaaababaa")]
    #[test_case(PART_2_RULES, "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa")]
    #[test_case(PART_2_RULES, "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba")]
    fn test_part_2_match(rules: &str, input: &str) {
        let rules = Rules::parse_str(rules).patch();
        assert!(rules.match_str(input))
    }
}
