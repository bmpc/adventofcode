mod utils;
use nalgebra::{Vector6, Matrix6};

const INPUT_FILE: &str = "./input/24_input.txt"; const TEST_AREA: (u64, u64) = (200000000000000, 400000000000000);
//const INPUT_FILE: &str = "./input/24_input_test.txt"; const TEST_AREA: (u64, u64) = (7, 27);

#[derive(Debug)]
struct Hailstone {
    _id: u32,
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    m: f64,
    c: f64
}

impl Hailstone {
    fn new(id: u32, x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) -> Self {
        let p1 = (x, y);
        let p2 = ((x + vx), (y + vy));

        let m: f64 = (p2.1 - p1.1) / (p2.0 - p1.0);
        let c: f64 = p1.1 - m * p1.0;

        Self { _id: id, x, y, z: z, vx, vy, vz: vz, m, c }
    }

    fn intersect(self: &Self, other: &Self) -> Option<(f64, f64)> {
        if self.m == other.m {
            // parallel lines (or the same line)
            None
        } else {
            let x = (self.c - other.c) / (other.m - self.m);
            let y = self.m * x + self.c;

            Some((x, y))
        }
    }
}

fn count_intersections(test_area: (u64, u64), hailstones: &Vec<Hailstone>) -> usize {
    let mut count = 0;

    for idx in 0..hailstones.len() {
        let hs = &hailstones[idx];
        if idx < hailstones.len() - 1 {
            for idx2 in (idx + 1)..hailstones.len() {
                let ihs = &hailstones[idx2];
                if let Some((x, y)) = hs.intersect(ihs) {
                    if x.floor() as u64 >= test_area.0 && y.floor() as u64 >= test_area.0 &&
                        x.ceil() as u64 <= test_area.1 && y.ceil() as u64 <= test_area.1 && 
                        // check if hailstones will intersect only in the future
                        (x - hs.x) * hs.vx >= 0.0 && 
                        (y - hs.y) * hs.vy >= 0.0 && 
                        (x - ihs.x) * ihs.vx >= 0.0 && 
                        (y - ihs.y) * ihs.vy >= 0.0 {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn calculate_rock_position(hailstones: &Vec<Hailstone>) -> (f64, f64, f64) {
    // considering only the first 3 hailstones
    let h0 = &hailstones[0];
    let h1 = &hailstones[1];
    let h2 = &hailstones[2];
   
    let matrix = Matrix6::new(
        0.0, h0.vz - h1.vz, h1.vy - h0.vy, 0.0, h1.z - h0.z, h0.y - h1.y,
		h1.vz - h0.vz, 0.0, h0.vx - h1.vx, h0.z - h1.z, 0.0, h1.x - h0.x,
		h0.vy - h1.vy, h1.vx - h0.vx, 0.0, h1.y - h0.y, h0.x - h1.x, 0.0,
		0.0, h0.vz - h2.vz, h2.vy - h0.vy, 0.0, h2.z - h0.z, h0.y - h2.y,
		h2.vz - h0.vz, 0.0, h0.vx - h2.vx, h0.z - h2.z, 0.0, h2.x - h0.x,
		h0.vy - h2.vy, h2.vx - h0.vx, 0.0, h2.y - h0.y, h0.x - h2.x, 0.0,
    );

    let eqs = Vector6::new(
        h0.y*h0.vz - h0.vy*h0.z - (h1.y*h1.vz - h1.vy*h1.z),
		h0.z*h0.vx - h0.vz*h0.x - (h1.z*h1.vx - h1.vz*h1.x),
		h0.x*h0.vy - h0.vx*h0.y - (h1.x*h1.vy - h1.vx*h1.y),
		h0.y*h0.vz - h0.vy*h0.z - (h2.y*h2.vz - h2.vy*h2.z),
		h0.z*h0.vx - h0.vz*h0.x - (h2.z*h2.vx - h2.vz*h2.x),
		h0.x*h0.vy - h0.vx*h0.y - (h2.x*h2.vy - h2.vx*h2.y),
    );

    let decomp = matrix.lu();
    let res = decomp.solve(&eqs).expect("Linear resolution failed.");

    (res[0], res[1], res[2])
}

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut hailstones: Vec<Hailstone> = vec![];

        let mut hailstone_id = 0;

        for line in lines {
            if let Ok(text) = line {
                let mut parts = text.split("@");
                let position = parts.next().unwrap();
                let velocity = parts.next().unwrap();

                let mut p = position.split(",");
                let x: f64 = p.next().unwrap().trim().parse().unwrap();
                let y: f64 = p.next().unwrap().trim().parse().unwrap();
                let z: f64 = p.next().unwrap().trim().parse().unwrap();

                let mut v = velocity.split(",");
                let vx: f64 = v.next().unwrap().trim().parse().unwrap();
                let vy: f64 = v.next().unwrap().trim().parse().unwrap();
                let vz: f64 = v.next().unwrap().trim().parse().unwrap();

                hailstones.push(Hailstone::new(hailstone_id, x, y, z, vx, vy, vz));
                hailstone_id += 1;
            }
        }

        let intersections = count_intersections(TEST_AREA, &hailstones);

        println!("[Part 1] Number of intersections that occur within the test area : {}", intersections);

        let rock = calculate_rock_position(&hailstones);

        println!("[Part 2] Sum of X, Y, and Z coordinates of the initial rock position: {}", (rock.0 + rock.1 + rock.2).floor());
        
    } else {
        eprintln!("Could not the hailstones from {}", INPUT_FILE);
    }

}
