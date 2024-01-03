mod utils;

use std::fmt;

const SPIN_CYCLES: u32 = 1100;
// const SPIN_CYCLES: u32 = 1000000000;

//const INPUT_FILE: &str = "./input/14_input_test.txt";
const INPUT_FILE: &str = "./input/14_input.txt";

struct ParabolicDish {
    width: usize,
    height: usize,
    data: Vec<char>
}

impl ParabolicDish {
    fn tilt_north(&mut self) {
        for col in 0..self.width {
            for row in 0..self.height {
                let ch = self.data[row*self.width + col];
                if ch == 'O' && row > 0 {
                    let mut i: i32 = (row - 1) as i32;
                    let mut first_empty = -1;
                    
                    while i >= 0 {
                        let cch = self.data[(i as usize) *self.width + col];
                        if cch != '.' {
                            break;
                        } else {
                            first_empty = i;
                        }

                        i -= 1;
                    }

                    if first_empty >= 0 {
                        self.data[(first_empty as usize)*self.width + col] = 'O';
                        self.data[row*self.width + col] = '.';
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for col in 0..self.width {
            for row in (0..self.height).rev() {
                let ch = self.data[row*self.width + col];
                if ch == 'O' && row < self.height - 1 {
                    let mut i = row + 1;
                    let mut first_empty: i32 = -1;
                    
                    while i < self.height {
                        let cch = self.data[(i as usize) *self.width + col];
                        if cch != '.' {
                            break;
                        } else {
                            first_empty = i as i32;
                        }

                        i += 1;
                    }

                    if first_empty >= 0 {
                        self.data[(first_empty as usize)*self.width + col] = 'O';
                        self.data[row*self.width + col] = '.';
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) { // right
        for row in 0..self.height {
            for col in (0..self.width).rev() {
                let ch = self.data[row*self.width + col];
                if ch == 'O' && col < self.width - 1 {
                    let mut i = col + 1;
                    let mut first_empty: i32 = -1;
                    
                    while i < self.width {
                        let cch = self.data[row*self.width + i];
                        if cch != '.' {
                            break;
                        } else {
                            first_empty = i as i32;
                        }

                        i += 1;
                    }

                    if first_empty >= 0 {
                        self.data[row*self.width + (first_empty as usize)] = 'O';
                        self.data[row*self.width + col] = '.';
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) { // right
        for row in 0..self.height {
            for col in 0..self.width {
                let ch = self.data[row*self.width + col];
                if ch == 'O' && col > 0 {
                    let mut i: i32 = (col - 1) as i32;
                    let mut first_empty: i32 = -1;
                    
                    while i >= 0 {
                        let cch = self.data[row*self.width + (i as usize)];
                        if cch != '.' {
                            break;
                        } else {
                            first_empty = i as i32;
                        }

                        i -= 1;
                    }

                    if first_empty >= 0 {
                        self.data[row*self.width + (first_empty as usize)] = 'O';
                        self.data[row*self.width + col] = '.';
                    }
                }
            }
        }
    }


    fn north_load(&self) -> u32 {
        let mut sum = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                let ch = self.data[row*self.width + col];
                if ch == 'O' {
                    sum += self.height - row;
                }
            }
        }
        sum as u32
    }

}

impl fmt::Display for ParabolicDish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for row in 0..self.height {
            for col in 0..self.width {
                let ch = self.data[row*self.width + col];
                s.push(ch);
            }
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut data: Vec<char> = vec![];
        let mut width = 0;
        let mut height = 0;

        for line in lines {
            if let Ok(text) = line {
                if width == 0 {
                    width = text.len();
                }
                data.extend(text.chars());
                height += 1;
            }
        }

        let mut dish = ParabolicDish { width, height, data: data.clone() };
        let mut dish2 = ParabolicDish { width, height, data: data.clone() };
        
        dish.tilt_north();
        let load = dish.north_load();
        println!("[Part 1] Total load of north support beams: {}", load);

        for _ in 1..SPIN_CYCLES {
            dish2.tilt_north();
            dish2.tilt_west();
            dish2.tilt_south();
            dish2.tilt_east();
            let load_after_cycles = dish2.north_load();
            println!("{}", load_after_cycles);
        }

        let load_after_cycles = dish2.north_load();
        
        //println!("{}", dish);

        println!("[Part 2] Total load of north support beams after {} cycles: {}", SPIN_CYCLES, load_after_cycles);
    } else {
        eprintln!("Could not load Parabolic Reflection Dish {}", INPUT_FILE);
    }

}
