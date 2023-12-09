mod utils;

static INPUT_FILE: &str = "./06_input.txt";

struct Race {
    time: u32,
    distance: u32
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

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut total: u32 = 1;

        let mut it = lines.into_iter();

        if let Ok(times) = it.nth(0).unwrap() {
            if let Ok(distances) = it.nth(0).unwrap() {
                let races = parse_races(&times, &distances);

                for race in races {
                    total *= count_records(&race);
                }
            }
        }

        println!("Multiplication of the number of ways the record is beat: {}", total);
    } else {
        eprintln!("Could not extract races from {}", INPUT_FILE);
    }

}
