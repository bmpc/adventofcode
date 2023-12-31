mod utils;

use std::env;
use std::collections::HashMap;

// const INPUT_FILE: &str = "./input/12_input_test.txt";
const INPUT_FILE: &str = "./input/12_input.txt";

#[derive(Debug)]
struct Record {
    pattern: String,
    damaged_groups: Vec<u32>
}

fn parse_record(text: &str) -> Record {
    let mut parts = text.split(' ');

    let pattern = parts.next().unwrap().trim().to_owned();
    let damaged_groups: Vec<u32> = parts.next().unwrap().trim().split(',').map(|n| n.parse().unwrap()).collect();
    
    Record { pattern, damaged_groups }
}

// naive solution -> brute force
fn count_arrangements_bf(pattern: String, groups: &Vec<u32>) -> u64 {
    let mut sum = 0;
    let s = pattern.clone();
    
    match s.find('?') {
        Some(_) => {
            sum += count_arrangements_bf(s.clone().replacen('?', "#", 1), groups);
            sum += count_arrangements_bf(s.clone().replacen('?', ".", 1), groups);
            sum
        }
        None => if pattern_matches(&s, groups) {
            1
        } else {
            0
        }
    }
}

fn pattern_matches(pattern: &str, groups: &Vec<u32>) -> bool {
    let mut count = 0;
    let mut g = 0;

    for (i, c) in pattern.chars().enumerate() {
        if c == '#' {
            if g == groups.len() {
                return false;
            }
            count += 1;
        }
        if (c == '.' || i + 1 == pattern.len()) && count > 0 {
            if g == groups.len() || count != groups[g] {
                return false;
            }

            g += 1;
            count = 0;
        }
    }

    //println!("{}", pattern);
    g >= groups.len()
}

fn count_arrangements_rec(pattern: &str, groups: &Vec<u32>) -> u64 {
    let mut ht: HashMap<String, u64> = HashMap::new();
    return _count_arrangements_rec(pattern, groups, &mut ht);
}

fn _count_arrangements_rec(pattern: &str, groups: &Vec<u32>, cache: &mut HashMap<String, u64>) -> u64 {
    if pattern.len() == 0 {
        if groups.len() == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    if groups.len() == 0 {
        if pattern.contains('#') {
            return 0;
        } else {
            return 1;
        }
    }

    let nums: String = groups.iter().map(|n| n.to_string() + "_").collect();
    let key = pattern.to_owned() + &nums;
    if cache.contains_key(key.as_str()) {
        return *(cache.get(key.as_str()).unwrap());
    }
 
    let mut result: u64 = 0;

    let first_ch = pattern.chars().nth(0).unwrap();

    if first_ch == '.' || first_ch == '?' {
        result += _count_arrangements_rec(&pattern[1..], groups, cache);
    }

    if first_ch == '#' || first_ch == '?' {
        if (groups[0] as usize) <= pattern.len() && !pattern[..groups[0] as usize].contains('.') && ((groups[0] as usize) == pattern.len() || pattern.chars().nth(groups[0] as usize).unwrap() != '#') {
            let next_pattern = if (groups[0] as usize) + 1 < pattern.len() {
                &pattern[((groups[0] as usize) + 1)..]
            } else {
                ""
            };

            result += _count_arrangements_rec(next_pattern, &groups[1..].to_vec(), cache);
        }
    }

    cache.insert(key, result);

    return result;


}

fn count_arrangements_imp(mut pattern: String, groups: &Vec<u32>) -> u64 {
    let mut sum = 0;

    pattern.push('.'); // HACK

    let mut patterns = vec![&pattern[..]];

    for (i, n) in groups.iter().enumerate() {
        let mut temp = Vec::new();
        for pattern in patterns {
            if pattern.len() > 0 {
                temp.append(&mut find_next_possible_arrangements(pattern, *n));
            }
        }

        if i == &groups.len() - 1 {
            for p in &temp {
                if !(*p).contains('#') {
                    sum += 1;
                }
            }
            //sum = temp.len();
        }

        patterns = temp;
    }

    sum
}

fn find_next_possible_arrangements(sub_pattern: &str, n: u32) -> Vec<&str> {    
    let mut sub: Vec<&str> = Vec::new();
    
    let mut pattern_queue = vec![sub_pattern];
    
    while !pattern_queue.is_empty() {
        let mut pounds = 0;
        let mut fixed = false;
        let pattern: &str = pattern_queue.pop().unwrap();

        for (i, ch) in pattern.char_indices() {
            if pounds == n && (ch == '.' || ch == '?') {
                sub.push(&pattern[(i + 1)..]);
                break;
            }

            if ch == '.' && pounds > 0 && pounds < n {
                pattern_queue = vec![];
                pounds = 0;
                if fixed {
                    break;
                }
            }

            if pounds == n && ch == '#' {
                // invalid
                break;
            }

            if ch == '?' {
                pounds += 1;
                if !fixed && pounds == 1 {
                    let next_pattern = &pattern[(i + 1)..];
                    if !pattern_queue.contains(&next_pattern) {
                        pattern_queue.push(next_pattern);
                    }
                }
            } else if ch == '#' {
                pounds += 1;
                fixed = true;
            }
        }

    }

    sub
}

fn unfold_record(record: &Record) -> Record {
    let groups = record.damaged_groups.repeat(5);
    let mut pattern = String::with_capacity(record.pattern.capacity() * 5);
    for i in 0..5 {
        pattern.push_str(&record.pattern);
        if i < 4 {
            pattern.push('?');
        }
    }
    
    Record { pattern, damaged_groups: groups }
}

fn count_arrangements_brute_force(records: &Vec<Record>) -> (u64, u64) {
    let sum1: u64 = records.iter()
            .map(|r| count_arrangements_bf(r.pattern.clone(), &r.damaged_groups))
            .sum();

    let sum2: u64 = records.iter().map(|r| {
        let rec = unfold_record(r);
        count_arrangements_bf(rec.pattern.clone(), &rec.damaged_groups)
    }).sum::<u64>();

    (sum1, sum2)
}

fn count_arrangements_imperative(records: &Vec<Record>) -> (u64, u64) {
    let sum1: u64 = records.iter()
            .map(|r| count_arrangements_imp(r.pattern.clone(), &r.damaged_groups))
            .sum();

    let sum2: u64 = records.iter().map(|r| {
        let rec = unfold_record(r);
        count_arrangements_imp(rec.pattern.clone(), &rec.damaged_groups)
    }).sum::<u64>();

    (sum1, sum2)
}

fn count_arrangements_recursive(records: &Vec<Record>) -> (u64, u64) {
    let sum1: u64 = records.iter()
        .map(|r| {
            count_arrangements_rec(r.pattern.as_str(), &r.damaged_groups)
        })
        .sum();

    let sum2: u64 = records.iter().map(|r| {
        let rec = unfold_record(r);
        count_arrangements_rec(rec.pattern.as_str(), &rec.damaged_groups)
    }).sum();

    (sum1, sum2)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let alg = &args[1];

    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut records = Vec::new();

        for line in lines {
            if let Ok(text) = line {
                records.push(parse_record(&text));
            }
        }

        let results = match alg.as_str() {
            "BRUTE" => count_arrangements_brute_force(&records),
            "IMP" => count_arrangements_imperative(&records),
            "REC" => count_arrangements_recursive(&records),
            _ => count_arrangements_recursive(&records)
        };

        println!("[Part 1] Sum of all operational spring arrangements: {}", results.0);
        println!("[Part 2] Sum of all unfolded operational spring arrangements: {}", results.1);
    } else {
        eprintln!("Could not load hot springs from {}", INPUT_FILE);
    }

}
