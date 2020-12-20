use std::error::Error;
use std::io::{self, BufRead};
use std::str::FromStr;

#[macro_use] extern crate lazy_static;
use regex::Regex;

enum Rule {
    None,
    Char(char),
    Ref(usize),
    Sequence(Vec<Rule>),
    Choice(Vec<Rule>),
}

impl Default for Rule {
    fn default() -> Self {
        Rule::None
    }
}

impl FromStr for Rule {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref CHAR_RE: Regex = Regex::new(r#"^"(\w)"$"#).unwrap();
            static ref REF_RE: Regex = Regex::new(r"^(\d+)$").unwrap();
            static ref CHOICE_SEPARATOR_RE: Regex = Regex::new(r"\s*\|\s*").unwrap();
            static ref SEQ_SEPARATOR_RE: Regex = Regex::new(r"\s+").unwrap();
        }

        if let Some(char_captures) = CHAR_RE.captures(s) {
            return Ok(Rule::Char(char_captures[1].chars().next().unwrap()));
        }

        if let Some(ref_captures) = REF_RE.captures(s) {
            return Ok(Rule::Ref(ref_captures[1].parse()?));
        }

        if CHOICE_SEPARATOR_RE.is_match(s) {
            return Ok(Rule::Choice(CHOICE_SEPARATOR_RE.split(s).map(|s| s.parse()).collect::<Result<_, _>>()?));
        }

        if SEQ_SEPARATOR_RE.is_match(s) {
            return Ok(Rule::Sequence(SEQ_SEPARATOR_RE.split(s).map(|s| s.parse()).collect::<Result<_, _>>()?));
        }

        return Err(Box::from(format!("Failed to parse string '{}'", s)));
    }
}

struct RuleWithIndex {
    index: usize,
    rule: Rule,
}

impl FromStr for RuleWithIndex {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RULE_RE: Regex = Regex::new(r"^(\d+):\s*([\S].*)$").unwrap();
        }

        match RULE_RE.captures(s) {
            Some(captures) => Ok(Self {
                index: captures[1].parse()?,
                rule: captures[2].parse()?,
            }),
            None => Err(Box::from(format!("Failed to parse string '{}'", s))),
        }
    }
}

fn get_match(rules: &[Rule], rule: &Rule, data: &[char], offset: usize) -> Vec<usize> {
    match rule {
        Rule::None => vec![],
        Rule::Char(ch) => {
            if offset < data.len() && data[offset] == *ch {
                vec![offset + 1]
            } else {
                vec![]
            }
        },
        Rule::Ref(rule_index) => get_match(&rules, &rules[*rule_index], &data, offset),
        Rule::Sequence(rules_sequence) => {
            let mut current_offsets = vec![offset];
            for subrule in rules_sequence {
                current_offsets = current_offsets.into_iter()
                    .flat_map(|current_offset| get_match(&rules, subrule, &data, current_offset))
                    .collect();
            }

            current_offsets
        },
        Rule::Choice(rules_choice) => rules_choice.iter()
            .flat_map(|subrule| get_match(&rules, subrule, &data, offset))
            .collect(),
    }
}

fn is_match(rules: &[Rule], rule: &Rule, data: &[char]) -> bool {
    get_match(&rules, &rule, &data, 0).contains(&data.len())
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().into_iter();
    let mut rules_lines: Vec<RuleWithIndex> = Vec::new();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line == "" {
            break;
        }

        rules_lines.push(line.parse().unwrap());
    }

    let mut rules: Vec<Rule> = Vec::new();
    rules.resize_with(
        rules_lines.iter().map(|rule_with_index| rule_with_index.index).max().unwrap() + 1,
        Default::default
    );
    for rule_line in rules_lines {
        rules[rule_line.index] = rule_line.rule;
    }

    let mut matches_count = 0;
    loop {
        match lines.next() {
            Some(Ok(line)) => {
                let result = is_match(&rules, &Rule::Ref(0), &line.chars().collect::<Vec<_>>());
                println!("Match: {}", result);
                if result {
                    matches_count += 1;
                }
            }
            _ => {
                break;
            }
        }
    }

    println!("{}", matches_count);
}
