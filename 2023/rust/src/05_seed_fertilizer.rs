mod utils;

static INPUT_FILE: &str = "./05_input.txt";
// static INPUT_FILE: &str = "./05_input_test.txt";

#[derive(Copy, Clone)]
struct Mapping {
    destination: u64,
    source: u64,
    length: u64
}

struct Category {
    mappings: Vec<Mapping>,
    start: u64,
    end: u64
}

impl Category {
    fn new() -> Self {
        Self {
            mappings: Vec::new(),
            start: u64::MAX,
            end: 0
        }
    }

    fn push_mapping(&mut self, mapping: Mapping) {
        self.mappings.push(mapping);

        if mapping.source < self.start {
            self.start = mapping.source;
        }

        let end = mapping.source + mapping.length;
        if end > self.end {
            self.end = end;
        }
    }
}

fn parse_seeds(text: &str) -> Vec<u64> {
    let seeds_txt = text[7..].trim().split(' ');

    seeds_txt.into_iter().map(|seed| seed.trim().parse::<u64>().unwrap()).collect()
}

fn parse_mapping(text: &str) -> Mapping {
    let mut parts = text.split(' ');
    let destination = parts.next().expect("No Destination Range found!").parse().unwrap();
    let source = parts.next().expect("No Source Range found!").parse().unwrap();
    let length = parts.next().expect("No Range length found!").parse().unwrap();

    Mapping { destination, source, length }
}

fn find_closest_seed_location(seed: u64, categories: &Vec<Category>) -> u64 {
    let mut step_seed = seed;

    for cat in categories {
        if step_seed >= cat.start && step_seed <= cat.end {
            for mapping in &cat.mappings {
                if step_seed >= mapping.source && step_seed < mapping.source + mapping.length {
                    step_seed = mapping.destination + (step_seed - mapping.source);
                    break;
                }
            }
        }
    }

    step_seed
}

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut lowest_loc_p1: u64 = u64::MAX;
        let mut lowest_loc_p2: u64 = u64::MAX;

        let mut seeds: Vec<u64> = Vec::new();
        let mut categories: Vec<Category> = Vec::new();

        for (i, line) in lines.enumerate() {
            if let Ok(text) = line {
                if i == 0 {
                    seeds = parse_seeds(&text);
                } else {
                    if !text.trim().is_empty() {
                        if text.contains("map:") {
                            categories.push(Category::new());
                        } else {
                            let mapping = parse_mapping(&text);
                            categories.last_mut().and_then(|cat| Some(cat.push_mapping(mapping)));
                        }
                    }
                }
            }
        }

        // part 1
        for seed in &seeds {
            let loc = find_closest_seed_location(*seed, &categories);
            if loc < lowest_loc_p1 {
                lowest_loc_p1 = loc;
            }
        }

        // part 2 (seeds are ranges)

        // Naive solution that takes about 40m to compute :(
        // An optimization was done where category mappings are skipped if the source is not in range.

        let seed_ranges = seeds.chunks(2).map(|x| (x[0], x[1])).collect::<Vec<_>>();
        let mut index = 0;
        for (seed_start, range) in seed_ranges {
            index+=1;
            println!("Processing seed range #{} [{}, {}]", index, seed_start, range);
            for seed in seed_start..(seed_start + range) {
                let loc = find_closest_seed_location(seed, &categories);

                if loc < lowest_loc_p2 {
                    lowest_loc_p2 = loc;
                }
            }
        }

        println!("[Part 1] Lowest location number: {}", lowest_loc_p1);
        println!("[Part 2] Lowest location number: {}", lowest_loc_p2);
    } else {
        eprintln!("Could not seed mappings from {}", INPUT_FILE);
    }

}
