mod utils;

static INPUT_FILE: &str = "./input/06_input.txt";
//static INPUT_FILE: &str = "./input/06_input_test.txt";

struct Race {
    time: u64,
    distance: u64
}

fn count_records(race: &Race) -> u32 {
    let mut count = 0;
    for x in 1..race.time {
        if (race.time - x)*x > race.distance {
            count+=1;
        }
    }

    count
}

fn parse_races(times: &str, distances: &str) -> Vec<Race> {
    let race_times = times[5..].trim().split(' ');
    let race_distances = distances[9..].trim().split(' ');

    race_times.into_iter()
        .filter(|r| !r.is_empty())
        .zip(race_distances.filter(|d| !d.is_empty()))
        .map(|(t,d)| Race { time: t.trim().parse().unwrap(), distance: d.trim().parse().unwrap()})
        .collect()
}

fn parse_single_race(times: &str, distances: &str) -> Race {
    let time = times[5..].trim().replace(' ', "").parse().unwrap();
    let distance = distances[9..].trim().replace(' ', "").parse().unwrap();

    Race {time, distance }
}

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut total1: u32 = 1;
        let mut total2: u32 = 1;

        let mut it = lines.into_iter();

        if let Ok(times) = it.nth(0).unwrap() {
            if let Ok(distances) = it.nth(0).unwrap() {

                // part 1 - multiple races
                let races = parse_races(&times, &distances);

                for race in races {
                    total1 *= count_records(&race);
                }

                // part 2 - single race
                let race = parse_single_race(&times, &distances);
                total2 = count_records(&race);

            }
        }

        println!("[Part 1] Multiplication of the number of ways the record is beat: {}", total1);
        println!("[Part 2] Number of ways the record is beat: {}", total2);
    } else {
        eprintln!("Could not extract races from {}", INPUT_FILE);
    }

}
