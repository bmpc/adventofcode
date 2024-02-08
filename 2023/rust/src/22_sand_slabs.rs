mod utils;

use std::cmp::Ordering;

//const INPUT_FILE: &str = "./input/22_input.txt";
const INPUT_FILE: &str = "./input/22_input_test.txt";

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Brick {
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct SettledBrick {
    begin: (u32, u32, u32),
    end: (u32, u32, u32),
    s_bricks: Vec<SettledBrick> // bricks in which this block is settled
}

impl SettledBrick {
    fn new(b: &Brick) -> Self {
        Self {
            begin: b.begin,
            end: b.end,
            s_bricks: vec![]
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

fn _intersect_bogus(b: &Brick, sb: &SettledBrick) -> bool {
    let (x1, y1) = (b.begin.0 as i32, b.begin.1 as i32);
    let (x2, y2) = (b.end.0 as i32, b.end.1 as i32);
    let (x3, y3) = (sb.begin.0 as i32, sb.begin.1 as i32);
    let (x4, y4) = (sb.end.0 as i32, sb.end.1 as i32);

    let a1 = if x1 - x2 != 0 { (y1 - y2) / (x1 - x2) } else { 0 };  // check dividing by zero
    let a2 = if x3 - x4 != 0 { (y3 - y4) / (x3 - x4) } else { 0 };  // check dividing by zero
    let b1 = y1 - a1 * x1; // = y2-a1 * x2;
    let b2 = y3 - a2 * x3; // = y4 - a2 * x4;

    if a1 == a2 {
        return false;
    }

    let xa = if a1 - a2 != 0 { (b2 - b1) / (a1 - a2) } else { 0 }; // check dividing by zero
    
    !(xa < x1.min(x2).max(x3.min(x4))) || (xa > x1.max(x2).min(x3.max(x4)))
}

fn settle_bricks(bricks: &Vec<Brick>) -> Vec<SettledBrick> {
    let mut settled_bricks: Vec<SettledBrick> = vec![];

    for brick in bricks {
        let mut z = 0;
        let mut s_bricks = vec![];
        for sb in settled_bricks.iter().rev() {
            if z != 0 && sb.end.2 != z {
                // exhausted all blocks at the intersect level
                break;
            }
            let intersec = brick.begin.2 > sb.begin.2 && intersect(&brick, sb);
            if intersec {
                z = sb.end.2;
                s_bricks.push(sb.clone());
            } else {
                println!("debug");
            }
        }

        if !s_bricks.is_empty() {
            let mut b = *brick;
            b.end.2 = b.end.2 - b.begin.2 + z + 1;
            b.begin.2 = z + 1;
            let mut sb = SettledBrick::new(&b);
            sb.s_bricks = s_bricks;

            println!("s_bricks_len: {}", sb.s_bricks.len());
            
            settled_bricks.push(sb);
        } else {
            // the block is in the ground
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
    }

    settled_bricks
}

fn disintegrable_bricks(bricks: &Vec<SettledBrick>) -> u32 {
    bricks.iter().fold(0, |sum, b| match b.s_bricks.len() {
        0 => sum + 1,
        ln if ln > 1 => ln as u32 + sum,
        _ => sum
    })
}

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut bricks: Vec<Brick> = vec![];

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

                bricks.push(Brick {begin: (x1, y1, z1), end: (x2, y2, z2)});
            }
        }

        bricks.sort();

        let settled_bricks = settle_bricks(&bricks);
     
        let sum1 = disintegrable_bricks(&settled_bricks);

        println!("[Part 1] Number of bricks that can be safely disintegrated : {}", sum1);
    } else {
        eprintln!("Could not the snapshot of bricks while falling from {}", INPUT_FILE);
    }

}
