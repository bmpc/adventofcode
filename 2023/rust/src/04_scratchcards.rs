mod utils;

static INPUT_FILE: &str = "./04_input.txt";

struct Card {
    id: u32,
    winning: Vec<u32>,
    values: Vec<u32>
}

impl Card {
    fn calc_points(&self) -> usize {
        let count = self.values.iter()
            .filter(|value| self.winning.contains(value))
            .count();
        if count < 2 {
            count
        } else {
            (2 as usize).pow((count - 1) as u32)
        }
    }

    fn count_matches(&self) -> usize {
        self.values.iter()
            .filter(|value| self.winning.contains(value))
            .count()
    }
}

fn parse_card(text: &str) -> Card {
    let mut parts = text.split(':');
    let card = parts.next().expect("No Card ID found!");
    let numbers = parts.next().expect("No Card Numbers found!");
    let mut sets = numbers.trim().split('|');

    let win_set = sets.next().expect("No Winning set found!");
    let winning = win_set.trim().split(' ')
        .filter(|num| !num.trim().is_empty())
        .map(|num| num.trim().parse().unwrap()).collect();

    let values_set = sets.next().expect("No Values set found!");
    let values = values_set.trim().split(' ')
        .filter(|num| !num.trim().is_empty())
        .map(|num| num.trim().parse().unwrap()).collect();

    let id = card[5..].trim().parse().unwrap();

    Card { id, winning, values }
}

fn update_copies(index: usize, copies: usize, scratchcards: &mut Vec<usize>) {
    let new_els:i32 = ((index + copies + 1) as i32) - scratchcards.len() as i32;
    if new_els > 0 {
        for _ in 0..new_els {
            scratchcards.push(1);
        }
    }

    for i in 1..=copies {
        scratchcards[index + i] += scratchcards[index];
    }
}

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut sum: usize = 0;

        let mut scratchcards: Vec<usize> = Vec::new();
        let mut index = 0;
        
        for line in lines {
            if let Ok(text) = line {
                let card = parse_card(&text);
                
                let points: usize = card.calc_points();
                sum += points;

                let matches = card.count_matches();
                update_copies(index, matches, &mut scratchcards);

                index += 1;

                //println!("Game {} = {}", card.id, points);
            }
        }

        let total_scratchcards: usize = scratchcards.iter().sum::<usize>();

        println!("Total scratchcard winning points: {}", sum);
        println!("Total scratchcards: {}", total_scratchcards);
    } else {
        eprintln!("Could not extract scratchcard values from {}", INPUT_FILE);
    }

}