use std::cmp::Ordering;

mod utils;

static INPUT_FILE: &str = "./input/07_input.txt";
// static INPUT_FILE: &str = "./input/07_input_test.txt";

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum Card {
    A = 13, 
    K = 12, 
    Q = 11,
    J = 10,
    T = 9, 
    _9 = 8,
    _8 = 7, 
    _7 = 6, 
    _6 = 5, 
    _5 = 4, 
    _4 = 3, 
    _3 = 2, 
    _2 = 1
}


type Err = ();

impl Card {
    fn from_char(input: char) -> Result<Card, Err> {
        match input {
            'A'  => Ok(Card::A),
            'K'  => Ok(Card::K),
            'Q'  => Ok(Card::Q),
            'J'  => Ok(Card::J),
            'T'  => Ok(Card::T),
            '9'  => Ok(Card::_9),
            '8'  => Ok(Card::_8),
            '7'  => Ok(Card::_7),
            '6'  => Ok(Card::_6),
            '5'  => Ok(Card::_5),
            '4'  => Ok(Card::_4),
            '3'  => Ok(Card::_3),
            '2'  => Ok(Card::_2),
            _      => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum Type {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1
}

#[derive(Debug, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u32
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(other) {
            Ordering::Equal
        } else {
            let self_type = self.get_type();
            let other_type = other.get_type();

            if self_type == other_type {
                let mut ord = Ordering::Equal;

                for i in 0..5 {
                    ord = self.cards[i].cmp(&other.cards[i]);
                    if ord != Ordering::Equal {
                        break;
                    }
                }

                ord
            } else {
                self_type.cmp(&other_type)
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        let mut eq = true;

        for i in 0..5 {
            if self.cards[i] as u32 != other.cards[i] as u32 {
                eq = false
            }
        }

        eq
    }
}

impl Hand {
    fn get_type(&self) -> Type {
        let mut cards_map: [usize; 13] = [0; 13];
        for card in &self.cards {
            cards_map[*card as usize - 1] += 1;
        }

        let mut found_two = false;
        let mut found_three = false;

        let mut _type = Type::HighCard;

        for count in cards_map {
            match count {
                5 => { _type = Type::FiveOfAKind; break; }
                4 => { _type = Type::FourOfAKind; break; }
                3 => { 
                    if found_two {
                        _type = Type::FullHouse;
                        break;
                    } else {
                        _type = Type::ThreeOfAKind; 
                        found_three = true; 
                    }
                }
                2 => { 
                    if found_three { 
                        _type = Type::FullHouse;
                        break;
                    } else if found_two { 
                        _type = Type::TwoPair;
                        break;
                    } else {
                        _type = Type::OnePair;
                        found_two = true;
                    }
                }
                _ => {}
            }
        }

        _type
    }
}

fn parse_hand(text: &str) -> Hand {
    let mut parts = text.split(' ');

    let cards = parts.next().expect("No Hand found!").chars()
        .map(|ch| Card::from_char(ch).unwrap())
        .collect::<Vec<Card>>()
        .try_into()
        .unwrap();
    let bid = parts.next().expect("No Bid found!").parse().unwrap();

    Hand { cards, bid }
}

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut hands = Vec::new();

        for line in lines {
            if let Ok(text) = line {
                let hand = parse_hand(&text);
                hands.push(hand);
            }
        }

        hands.sort();

        let mut sum = 0;

        for (i, hand) in hands.iter().enumerate() {
            //println!("{:?} -> {:?}", hand, hand.get_type());
            sum += (i + 1) as u32 * hand.bid;
        }

        println!("[Part 1] Total winnings: {}", sum);
    } else {
        eprintln!("Could not Camel Cards from {}", INPUT_FILE);
    }

}
