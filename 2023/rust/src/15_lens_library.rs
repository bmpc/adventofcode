use std::fs;

// const INPUT_FILE: &str = "./input/15_input_test.txt";
const INPUT_FILE: &str = "./input/15_input.txt";

fn hash(value: &str) -> u32 {
    value.chars().fold(0, |curr, ch| (((curr + ch as u32) * 17) % 256))
}

const fn vec_init() -> Vec<&'static str> {
    Vec::new()
}

fn main() {
    if let Ok(content) = fs::read_to_string(INPUT_FILE) {
        let mut sum1: u32 = 0;
        let mut sum2: usize = 0;

        const VAL: Vec<&str> = vec_init();
        let mut boxes: [Vec<&str>; 256] = [VAL; 256];

        let steps = content.trim().split(",");
        for step in steps {
            if step.ends_with("-") {
                let label = &step[0..(step.len() - 1)];
                let hashed = hash(label);
                let v = &mut boxes[hashed as usize];
                v.retain(|st| !st.starts_with(label));
            } else {
                let mut it = step.split("=");
                let label = it.next().unwrap();
                let hashed = hash(label);
                let v = &mut boxes[hashed as usize];
                if let Some(pos) = v.iter().position(|st| st.starts_with(label)) {
                    v[pos] = step;
                } else {
                    v.push(step);
                }
            }

            sum1 += hash(step);
        }

        for (i, v) in boxes.iter().enumerate() {
            for (j, lens) in v.iter().enumerate() {
                let mut it = lens.split("=");
                it.next();
                let focal: usize = it.next().unwrap().parse().unwrap();
                sum2 += (1 + i) * (j + 1) * focal;
            }
        }
        
        println!("[Part 1] Sum of all step hashes: {}", sum1);
        println!("[Part 2] Focusing power of the resulting lens configuration: {}", sum2);
    } else {
        eprintln!("Could not initialization sequence from {}", INPUT_FILE);
    }

}
