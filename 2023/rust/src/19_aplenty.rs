mod utils;

use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Range;

const INPUT_FILE: &str = "./input/19_input.txt";
// const INPUT_FILE: &str = "./input/19_input_test.txt";

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Rule {
    rating: char,
    op: char,
    val: u32,
    res: String
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct Part {
    x: u32, 
    m: u32, 
    a: u32,
    s: u32
}

fn parse_wf(text: &str) -> (String, Vec<Rule>) {
    let mut s1 = text.split("{");
    let name = s1.next().unwrap().to_string();
    let s2 = s1.next().unwrap().strip_suffix("}").unwrap().split(",");
    let rules = s2.map(|r| parse_rule(r)).collect();
    
    (name, rules)
}

fn parse_rule(text: &str) -> Rule {
    let mut s = text.split([':']);

    let s1 = s.next().unwrap();
    let s2op = s.next();

    if let Some(s2) = s2op {
        let sb = s1.as_bytes();
        let rating = sb[0] as char;
        let op = sb[1] as char;
        let val = s1[2..].parse().unwrap();

        let res = s2.to_string();

        Rule { rating, op, val, res }
    } else {
        let res = s1.to_string();

        Rule { rating: '.', op: '.', val: 0, res }
    }
}

fn parse_part(text: &str) -> Part {
    let ratings = text.strip_prefix("{").unwrap().strip_suffix("}").unwrap().split(",");
    let mut ps = vec![0, 0, 0, 0];
    let mut i = 0;
    for rat in ratings {
        ps[i] = rat[2..].parse().unwrap();
        i += 1;
    }

    Part { x: ps[0], m: ps[1], a: ps[2], s: ps[3] }
}

fn is_accepted_part(part: Part, workflows: &HashMap<String, Vec<Rule>>) -> bool {
    let mut cr = "in";

    while cr != "A" && cr != "R" {
        for r in &workflows[cr] {
            let m = match r.rating {
                'x' => match r.op {
                    '>' if part.x > r.val => true,
                    '<' if part.x < r.val => true,
                    _ => false
                },
                'm' => match r.op {
                    '>' if part.m > r.val => true,
                    '<' if part.m < r.val => true,
                    _ => false
                },
                'a' => match r.op {
                    '>' if part.a > r.val => true,
                    '<' if part.a < r.val => true,
                    _ => false
                }
                's' => match r.op {
                    '>' if part.s > r.val => true,
                    '<' if part.s < r.val => true,
                    _ => false
                }
                '.' => true,
                _ => unreachable!()
            };

            if m {
                cr = &r.res;
                break;
            }
        }
    }

    cr == "A"
} 

fn match_range(r: Rule, range: Range<u32>) -> (Range<u32>, Range<u32>) {
    match r.op {
        '>' => {
            let nr = (r.val + 1)..range.clone().end;
            let rr = range.clone().start..(r.val + 1);
            (nr, rr)
        },
        '<' => {
            let nr = range.clone().start..r.val;
            let rr = r.val..range.clone().end;
            (nr, rr)
        },
        _ => unreachable!()
    }
}

fn find_accepted_ranges(wf: &str, mut ranges: Vec<Range<u32>>, workflows: &HashMap<String, Vec<Rule>>) -> u64 {
    if wf == "A" {
        // println!("Ranges: {:?}", ranges);

        return ranges.iter().map(|r| (r.end - r.start) as u64).reduce(|acc, v| acc * v).unwrap();
    }

    if wf == "R" {
        return 0;
    }

    let mut sum = 0;

    for r in &workflows[wf] {
        let ri_op = match r.rating {
            'x' => Some(0),
            'm' => Some(1),
            'a' => Some(2),
            's' => Some(3),
            _ => None
        };

        if let Some(ri) = ri_op  {
            let (nr, rr) = match_range(r.clone(), ranges[ri].clone());
            ranges[ri] = rr;
            let mut n_ranges = ranges.clone();
            n_ranges[ri] = nr;
            sum += find_accepted_ranges(&r.res, n_ranges, workflows);
        } else {
            sum += find_accepted_ranges(&r.res, ranges.clone(), workflows);
        }
    }

    sum
}

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
        let mut parts: Vec<Part> = vec![];

        let mut wfs = true;

        for line in lines {
            if let Ok(text) = line {
                if text.trim().is_empty() {
                    wfs = false;
                    continue;
                }

                if wfs {
                    let (name, rules) = parse_wf(&text);
                    workflows.insert(name, rules);
                } else {
                    let part = parse_part(&text);
                    parts.push(part);
                }
           }
        }
        
        let sum1: u32 = parts.iter().filter(|p| is_accepted_part(**p, &workflows)).map(|p| p.x + p.m + p.a + p.s).sum();

        println!("[Part 1] Sum of accepted part ratings : {}", sum1);

        let sum2 = find_accepted_ranges("in", vec![1..4001, 1..4001, 1..4001, 1..4001], &workflows);

        println!("[Part 2] Combinations of accepted rating ranges : {}", sum2);
    } else {
        eprintln!("Could not load parameters map from {}", INPUT_FILE);
    }

}
