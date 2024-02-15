mod utils;

const INPUT_FILE: &str = "./input/24_input.txt"; const TEST_AREA: (u64, u64) = (200000000000000, 400000000000000);
//const INPUT_FILE: &str = "./input/24_input_test.txt"; const TEST_AREA: (u64, u64) = (7, 27);

#[derive(Debug)]
struct Hailstone {
    _id: u32,
    x: i64,
    y: i64,
    _z: i64,
    vx: i64,
    vy: i64,
    _vz: i64,
    m: f64,
    c: f64
}

impl Hailstone {
    fn new(id: u32, x: i64, y: i64, z: i64, vx: i64, vy: i64, vz: i64) -> Self {
        let p1 = (x as f64, y as f64);
        let p2 = ((x + vx) as f64, (y + vy) as f64);

        let m: f64 = (p2.1 - p1.1) / (p2.0 - p1.0);
        let c: f64 = p1.1 - m * p1.0;

        Self { _id: id, x, y, _z: z, vx, vy, _vz: vz, m, c }
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
                        (x - hs.x as f64) * hs.vx as f64 >= 0.0 && 
                        (y - hs.y as f64) * hs.vy as f64 >= 0.0 && 
                        (x - ihs.x as f64) * ihs.vx as f64 >= 0.0 && 
                        (y - ihs.y as f64) * ihs.vy as f64 >= 0.0 {
                        count += 1;
                    }
                }
            }
        }
    }

    count
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
                let x: i64 = p.next().unwrap().trim().parse().unwrap();
                let y: i64 = p.next().unwrap().trim().parse().unwrap();
                let z: i64 = p.next().unwrap().trim().parse().unwrap();

                let mut v = velocity.split(",");
                let vx: i64 = v.next().unwrap().trim().parse().unwrap();
                let vy: i64 = v.next().unwrap().trim().parse().unwrap();
                let vz: i64 = v.next().unwrap().trim().parse().unwrap();

                hailstones.push(Hailstone::new(hailstone_id, x, y, z, vx, vy, vz));
                hailstone_id += 1;
            }
        }

        let intersections = count_intersections(TEST_AREA, &hailstones);

        println!("[Part 1] Number of intersections that occur within the test area : {}", intersections);
        
    } else {
        eprintln!("Could not the hailstones from {}", INPUT_FILE);
    }

}
