mod utils;

static INPUT_FILE: &str = "./05_input.txt";

struct Mapping {
    destination: u64,
    source: u64,
    length: u64
}

fn parse_seeds(text: &str) -> Vec<u32> {
    let seeds_txt = text[7..].trim().split(' ');

    seeds_txt.into_iter().map(|seed| seed.trim().parse::<u32>().unwrap()).collect()
}

fn parse_mapping(text: &str) -> Mapping {
    let mut parts = text.split(' ');
    let destination = parts.next().expect("No Destination Range found!").parse().unwrap();
    let source = parts.next().expect("No Source Range found!").parse().unwrap();
    let length = parts.next().expect("No Range length found!").parse().unwrap();

    Mapping { destination, source, length }
}

fn find_closest_seed_location(seed: u64, categories: &Vec<Vec<Mapping>>) -> u64 {

    let mut step_seed = seed;

    for cat in categories {
        for mapping in cat {
            if step_seed >= mapping.source && step_seed < mapping.source + mapping.length {
                step_seed = mapping.destination + (step_seed - mapping.source);
                break;
            }
        }
    }

    step_seed
}

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut lowest_loc: u64 = u64::MAX;

        let mut seeds: Vec<u32> = Vec::new();
        let mut categories: Vec<Vec<Mapping>> = Vec::new();

        for (i, line) in lines.enumerate() {
            if let Ok(text) = line {
                if i == 0 {
                    seeds = parse_seeds(&text);
                } else {
                    if !text.trim().is_empty() {
                        if text.contains("map:") {
                            categories.push(Vec::new());
                        } else {
                            let mapping = parse_mapping(&text);
                            categories.last_mut().and_then(|cat| Some(cat.push(mapping)));
                        }
                    }
                }
            }
        }

        for seed in seeds {
            let loc = find_closest_seed_location(seed as u64, &categories);
            if loc < lowest_loc {
                lowest_loc = loc;
            }
        }

        println!("Lowest location number: {}", lowest_loc);
    } else {
        eprintln!("Could not seed mappings from {}", INPUT_FILE);
    }

}
