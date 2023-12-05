use std::fmt::Display;
use std::cmp::Ordering;

mod utils;

static INPUT_FILE: &str = "./02_input.txt";

#[derive(Eq)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(other) {
            Ordering::Equal
        } else {
            let red_ord = self.red.cmp(&other.red);
            let green_ord = self.green.cmp(&other.green);
            let blue_ord = self.blue.cmp(&other.blue);
          
            if (red_ord == Ordering::Less || red_ord == Ordering::Equal) && 
                (green_ord == Ordering::Less || green_ord == Ordering::Equal) &&
                (blue_ord == Ordering::Less || blue_ord == Ordering::Equal) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && 
        self.green == other.green && 
        self.blue == other.blue
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Game {} -> (red: {}, green: {}, blue: {})", self.id, self.red, self.green, self.blue)
    }
}

fn parse_game(text: &str) -> Game {
    let mut red_max = 0;
    let mut green_max = 0;
    let mut blue_max = 0;
    
    let mut parts = text.split(':');
    let game = parts.next().expect("No Game ID found!");
    let body = parts.next().expect("No Game Cubes found!");
    let sets = body.split(';');
    for set in sets {
        let cubes = set.split(',');
        for cube in cubes {
            let mut cube_parts = cube.trim().split(' ');
            let count = cube_parts.next().expect("No Cube count found!");
            let color = cube_parts.next().expect("No Cube color found!");

            let count = count.parse::<u32>().unwrap();

            match color {
                "red" => if count > red_max { red_max = count },
                "green" => if count > green_max { green_max = count },
                "blue" => if count > blue_max { blue_max = count },
                _ => unreachable!("Invalid Color '{}'", color)
            }
        }
    }

    let id = game[5..].parse().unwrap();

    Game { id, red: red_max, green: green_max, blue: blue_max }
}

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut sum_valid: u32 = 0;
        let mut sum_power: u32 = 0;

        // 12 red cubes, 13 green cubes, and 14 blue cubes
        let test_game = Game { id: 0, red: 12, green: 13, blue: 14 };

        for line in lines {
            if let Ok(text) = line {
                let game = parse_game(&text);

                let power = game.red * game.green * game.blue;
                sum_power += power;

                if game <= test_game {
                    sum_valid += game.id;
                }

                //println!("{} is {}", game, if game <= test_game { "VALID" } else { "INVALID" });
            }
        }

        println!("Sum of valid Game IDs: {}", sum_valid);
        println!("Sum of powers: {}", sum_power);
    } else {
        eprintln!("Could not extract games from {}", INPUT_FILE);
    }

}
