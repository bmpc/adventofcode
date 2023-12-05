mod utils;

static CONVERT_NUMBERS: bool = true;

static INPUT_FILE: &str = "./01_input.txt";
static NUMBERS: &[(&str, &str)] = &[("1", "one"), ("2", "two"), ("3", "three"), ("4", "four"), ("5", "five"), ("6", "six"), ("7", "seven"), ("8", "eight"), ("9", "nine")];

fn convert_spelled_numbers_to_digits(text: &str) -> String {
    let dup = NUMBERS.iter().fold(text.to_owned(), |acc, num| acc.replace(num.1, &(num.1.to_owned() + num.1)));

    NUMBERS.iter().fold(dup, |acc, num| acc.replace(num.1, num.0))
}

fn get_value(text: &str) -> u8 {
    let mut first_dig: Option<char> = None;
    let mut last_dig: Option<char> = None; 
    
    for c in text.chars() { 
        if c.is_digit(10) {
            match first_dig {
                Some(_) => last_dig = Some(c),
                None => {
                    first_dig = Some(c);
                    last_dig = Some(c);
                }
            }
        }
    }

    if let Some(digit1) = first_dig {
        (digit1.to_digit(10).unwrap() * 10 + last_dig.unwrap().to_digit(10).unwrap()) as u8
    } else {
        0
    }
 }

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut sum: u32 = 0;
        for line in lines {
            if let Ok(text) = line {
                let value: u8 = if CONVERT_NUMBERS == true {
                    let converted_text: String = convert_spelled_numbers_to_digits(&text);
                    get_value(&converted_text)
                } else {
                    get_value(&text)
                };

                // println!("{} = {}", &text, &value);

                sum += value as u32;
            }
        }

        println!("Sum of all calibration values: {}", sum);
    } else {
        eprintln!("Could not extract codes from {}", INPUT_FILE);
    }

}
