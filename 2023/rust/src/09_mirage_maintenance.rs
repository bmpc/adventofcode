mod utils;

// const INPUT_FILE: &str = "./input/09_input_test.txt";
const INPUT_FILE: &str = "./input/09_input.txt";

fn extrapolate_next(sequence: &Vec<i32>) -> i32 {
    let mut reduced = false;
    let mut n = *sequence.last().unwrap();
    
    let mut seq = sequence.clone();
    
    while !reduced {
        let mut sub_seq = Vec::new();
        for i in 1..seq.len() {
            sub_seq.push(seq[i] - seq[i - 1]);
        }
        reduced = sub_seq.iter().all(|i| *i == 0);
        n += sub_seq.last().unwrap();

        seq = sub_seq;
    }
    
    n
}

fn extrapolate_prev(sequence: &Vec<i32>) -> i32 {
    let mut reduced = false;
    
    let mut seq = sequence.clone();
    let mut values = Vec::new();
    
    while !reduced {
        let mut sub_seq = Vec::new();
        for i in 1..seq.len() {
            sub_seq.push(seq[i] - seq[i - 1]);
        }
        reduced = sub_seq.iter().all(|i| *i == 0);

        values.push(*sub_seq.first().unwrap());

        seq = sub_seq;
    }

    let mut n = 0;
    for v in values.iter().rev() {
        n = v - n;
    }
  
    *sequence.first().unwrap() - n
}


fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut sum_next: i32 = 0;
        let mut sum_prev: i32 = 0;
        for line in lines {
            if let Ok(text) = line {
                let sequence = text.split(' ').map(|v|v.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                sum_next += extrapolate_next(&sequence);
                sum_prev += extrapolate_prev(&sequence);
            }
        }

        println!("[Part 1] Sum of next extrapolated values: {}", sum_next);
        println!("[Part 2] Sum of prev extrapolated values: {}", sum_prev);
    } else {
        eprintln!("Could not extract values from {}", INPUT_FILE);
    }

}
