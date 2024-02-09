mod utils;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

const INPUT_FILE: &str = "./input/22_input.txt";
// const INPUT_FILE: &str = "./input/22_input_test.txt";

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Brick {
    id: u32,
    begin: (u32, u32, u32),
    end: (u32, u32, u32)
}

// implement ordering based on z
impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.begin.2.partial_cmp(&other.begin.2)
    }
}
impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.begin.2.cmp(&other.begin.2)
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct SettledBrick {
    id: u32,
    begin: (u32, u32, u32),
    end: (u32, u32, u32),
    top: Vec<u32>,
    bottom: Vec<u32>
}

// implement ordering based on z
impl PartialOrd for SettledBrick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.end.2.partial_cmp(&self.end.2)
    }
}
impl Ord for SettledBrick {
    fn cmp(&self, other: &Self) -> Ordering {
        other.end.2.cmp(&self.end.2)
    }
}

impl SettledBrick {
    fn new(b: &Brick) -> Self {
        Self {
            id: b.id,
            begin: b.begin,
            end: b.end,
            top: vec![],
            bottom: vec![]
        }
    }
}

// Given three collinear points p, q, r, the function checks if 
// point q lies on line segment 'pr' 
fn on_segment(p: (i32, i32), q: (i32, i32), r: (i32, i32)) -> bool { 
    q.0 <= p.0.max(r.0) && q.0 >= p.0.min(r.0) && 
        q.1 <= p.1.max(r.1) && q.1 >= p.1.min(r.1)
}

// To find orientation of ordered triplet (p, q, r). 
// The function returns following values 
// 0 --> p, q and r are collinear 
// 1 --> Clockwise 
// 2 --> Counterclockwise 
fn orientation(p: (i32, i32), q: (i32, i32), r: (i32, i32)) -> i32 {
    // See https://www.geeksforgeeks.org/orientation-3-ordered-points/ 
    // for details of below formula. 
    let val = (q.1 - p.1) * (r.0 - q.0) - (q.0 - p.0) * (r.1 - q.1); 
  
    match val {
        0 => 0,                 // collinear
        x if x > 0 => 1,   // clockwise
        _ => 2 // if x < 0      // counter-clock wise
    }
}

// The main function that returns true if line segment 'p1q1' 
// and 'p2q2' intersect. 
fn intersect(b: &Brick, sb: &SettledBrick) -> bool {
    let p1 = (b.begin.0 as i32, b.begin.1 as i32);
    let q1 = (b.end.0 as i32, b.end.1 as i32);
    let p2 = (sb.begin.0 as i32, sb.begin.1 as i32);
    let q2 = (sb.end.0 as i32, sb.end.1 as i32);

    // Find the four orientations needed for general and 
    // special cases 
    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    // General case
    if o1 != o2 && o3 != o4 {
        return true;
    }

    // Special Cases 
    // p1, q1 and p2 are collinear and p2 lies on segment p1q1 
    if o1 == 0 && on_segment(p1, p2, q1) { return true; }
  
    // p1, q1 and q2 are collinear and q2 lies on segment p1q1 
    if o2 == 0 && on_segment(p1, q2, q1) { return true; }
  
    // p2, q2 and p1 are collinear and p1 lies on segment p2q2 
    if o3 == 0 && on_segment(p2, p1, q2) { return true; }
  
    // p2, q2 and q1 are collinear and q1 lies on segment p2q2 
    if o4 == 0 && on_segment(p2, q1, q2) { return true; }
  
    return false; // Doesn't fall in any of the above cases 
} 

fn settle_bricks(bricks: &Vec<Brick>) -> Vec<SettledBrick> {
    let mut settled_bricks: Vec<SettledBrick> = Vec::new();

    for brick in bricks {
        let mut z = 0;
        let mut bottom_bricks = vec![];

        for sb in settled_bricks.iter_mut() {
            if z != 0 && sb.end.2 != z {
                // exhausted all blocks at the intersect level
                break;
            }
            let intersec = brick.begin.2 > sb.end.2 && intersect(&brick, &sb); 
            if intersec {
                sb.top.push(brick.id);
                z = sb.end.2;
                bottom_bricks.push(sb.id);
            }
        }

        if !bottom_bricks.is_empty() {
            let mut b = *brick;
            b.end.2 = b.end.2 - b.begin.2 + z + 1;
            b.begin.2 = z + 1;
            let mut nsb = SettledBrick::new(&b);
            nsb.bottom = bottom_bricks;

            settled_bricks.push(nsb);
        } else {
            // the brick is in the ground
            let mut nb = *brick;
            if !(nb.begin.2 == 1 || nb.end.2 == 1) {
                if nb.begin.2 == nb.end.2 {
                    nb.begin.2 = 1;
                    nb.end.2 = 1;
                } else {
                    nb.end.2 = nb.end.2 - nb.begin.2 + 1;
                    nb.begin.2 = 1;
                }
            }
            settled_bricks.push(SettledBrick::new(&nb));
        }

        settled_bricks.sort();
    }

    settled_bricks
}

fn disintegrable_bricks(bricks: &Vec<SettledBrick>) -> HashMap<u32, &SettledBrick> {
    let bricks_map: HashMap<u32, SettledBrick> = bricks.into_iter().map(|b| (b.id, b.clone())).collect();

    let mut disintegrable_bricks_map = HashMap::new();

    for b in bricks {
        match b.top.len() {
            ln if ln > 1 => {
                let can_disintegrate = b.top.iter().map(|tid| bricks_map.get(tid).unwrap()).all(|bt| bt.bottom.len() > 1);
                if can_disintegrate {
                    disintegrable_bricks_map.insert(b.id, b);
                }
            },
            1 => {
                let tb = bricks_map.get(b.top.get(0).unwrap()).unwrap();
                if tb.bottom.len() > 1 {
                    disintegrable_bricks_map.insert(b.id, b);
                }
            },
            _ => { // zero
                disintegrable_bricks_map.insert(b.id, b);
             }
        };
    }

    disintegrable_bricks_map
}

fn count_fall_bricks(
    sb: &SettledBrick, 
    bricks_map: &HashMap<u32, SettledBrick>,
    fall: &mut HashSet<u32>) {

    for tsbid in &sb.top {
        let tsb = bricks_map.get(tsbid).unwrap();
        if tsb.bottom.len() > 1 {
            // if a top brick is being hold by other bricks, we need to check if these will also fall
            if tsb.bottom.iter().filter(|sbid| *sbid != tsbid).all(|id| fall.contains(id)) {
                fall.insert(tsb.id);
                count_fall_bricks(tsb, bricks_map, fall);
            }
        } else {
            fall.insert(tsb.id);
            count_fall_bricks(tsb, bricks_map, fall);
        }
    }
}

fn sum_fall_bricks(bricks: &Vec<SettledBrick>, disintegrable_bricks: &HashMap<u32, &SettledBrick>) -> usize {
    let mut sum :usize = 0;

    let bricks_map: HashMap<u32, SettledBrick> = bricks.into_iter().map(|b| (b.id, b.clone())).collect();
    
    for sb in bricks {
        if !disintegrable_bricks.contains_key(&sb.id) {
            let mut fall = HashSet::new();
            count_fall_bricks(sb, &bricks_map, &mut fall);
            sum += fall.len();
        }
    }

    sum
}

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut bricks: Vec<Brick> = vec![];

        let mut brick_id = 0;

        for line in lines {
            if let Ok(text) = line {
                let mut parts = text.split("~");
                let rb = parts.next().unwrap();
                let re = parts.next().unwrap();

                let mut pb = rb.split(",");
                let x1: u32 = pb.next().unwrap().parse().unwrap();
                let y1: u32 = pb.next().unwrap().parse().unwrap();
                let z1: u32 = pb.next().unwrap().parse().unwrap();

                let mut pe = re.split(",");
                let x2: u32 = pe.next().unwrap().parse().unwrap();
                let y2: u32 = pe.next().unwrap().parse().unwrap();
                let z2: u32 = pe.next().unwrap().parse().unwrap();

                bricks.push(Brick {id: brick_id, begin: (x1, y1, z1), end: (x2, y2, z2)});
                brick_id += 1;
            }
        }

        bricks.sort();

        let settled_bricks = settle_bricks(&bricks);
     
        let disintegrable_bricks = disintegrable_bricks(&settled_bricks);

        println!("[Part 1] Number of bricks that can be safely disintegrated : {}", disintegrable_bricks.len());

        let sum2 = sum_fall_bricks(&settled_bricks, &disintegrable_bricks);

        println!("[Part 2] Sum of the number of other bricks that would fall for each disintegrated brick : {}", sum2);
    } else {
        eprintln!("Could not the snapshot of bricks while falling from {}", INPUT_FILE);
    }

}
