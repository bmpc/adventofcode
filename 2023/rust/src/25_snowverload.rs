/**
 * Finding the minimum cut: https://en.wikipedia.org/wiki/Minimum_cut.
 * 
 * This is based on @maneatingape's solution: 
 *  - https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2023/day25.rs
 *  - https://bit.ly/3SUvYhv
 * 
 * If we find the begin and end nodes of the graph (where the distance between begin and end is greater), 
 * and we know that there are 3 edges that can be cut to form two distinct sub-graphs, then if we go
 * from the begin to the end nodes using 3 different paths, no other paths can reach the end node. 
 * The 4th iteration only reaches the the nodes in the first sub-graph. We can then compute the final 
 * answer.
 * 
 * Although this is a deterministic solution, it assumes that the minimum cut results in two graphs 
 * with similar number of nodes, which seems to be the case of the aoc inputs.
 * 
 *           * *       * *
 *         * * * * - * * * *
 *       S * * * * - * * * * E
 *         * * * * - * * * *
 *           * *       * *
 * 
 * However, if this was not the case, this solution will fail as the start and end nodes might be in 
 * the same sub-graph.
 * 
 *                S *
 *              * * * *
 *              * * * *
 *              * * * *
 *              * * * *
 *        * * - * * * * *
 *      * * * - A * * * * *
 *        * * - * * * * *
 *              * * * *
 *              * * * *
 *              * * * *
 *              * * * *
 *                * E
 *      
 * I've included a first naive attempt (_split_components2) which is a brute force approach where the 
 * combination of all edges are considered.
 * 
 */

mod utils;

use std::collections::{HashMap, HashSet, VecDeque};
use indexmap::IndexSet;

const INPUT_FILE: &str = "./input/25_input.txt"; 
//const INPUT_FILE: &str = "./input/25_input_test.txt";

#[derive(Debug, Hash, Copy, Clone, Eq)]
struct Edge<'a> {
    a: &'a str,
    b: &'a str
}

// implement Link equality independent of a,b order
impl<'a> PartialEq for Edge<'a> {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a &&
        self.b == other.b) || 
        (self.a == other.b &&
        self.b == other.a)
    }
}

// dumb solution (only works for the test input)
fn _split_components2(map: &HashMap<String, Vec<String>>) -> usize {
    // get all links
    let mut links: IndexSet<Edge> = IndexSet::new();
    
    for (comp, comp_links) in map {
        for l in comp_links {
            links.insert(Edge {a: comp, b: l});
        }
    }

    for i in 0..links.len() {
        for j in (i + 1)..links.len() {
            for k in (j + 1)..links.len() {
                let l1 = links.get_index(i).unwrap();
                let l2 = links.get_index(j).unwrap();
                let l3 = links.get_index(k).unwrap();

                let reachable = _check_broken_chain((l1, l2, l3), map);
                if reachable != map.len() {
                    return reachable * (map.len() - reachable);
                }
            }
        }
    }

    0
}

// part of dumb solution
fn _check_broken_chain(
    (l1, l2, l3): (&Edge, &Edge, &Edge), 
    components: &HashMap<String, Vec<String>>) -> usize {
    
    // start element
    let mut start = None;
    for (k, links) in components {
        for l in links {
            let link = Edge {a: k, b: l};
            if link.ne(l1) && link.ne(l2) && link.ne(l3) {
                start = Some(k);
                break;
            }
        }
        if start.is_some() {
            break;
        }
    }
    
    let mut seen: HashSet<&String> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start.unwrap());

    while let Some(comp) = queue.pop_front() {
        seen.insert(comp);

        let links = components.get(comp).unwrap();
        for l in links {
            let link = Edge {a: comp, b: l};
            if !seen.contains(l) && link.ne(l1) && link.ne(l2) && link.ne(l3) {
                queue.push_back(l);
            }
        }
    }

    seen.len()

}

fn split_components(map: &HashMap<String, Vec<String>>) -> usize {

    let k = map.keys().next().unwrap();

    // arbitrarily pick a node from the graph and then find the furthest node from it
    let start = furthest(k, map);
    // find the furthest node from the start node
    let end = furthest(start, map);
    
    // Iterate through the graph 3 times to find the min-cut. The 4th iteration will give us 
    // the nodes in the first sub-graph
    let size = process(start, end, map);
    size * (map.len() - size)
}

/// BFS across the graph to find the furthest nodes from start.
fn furthest<'a>(start: &'a str, map: &'a HashMap<String, Vec<String>>) -> &'a str {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut seen = HashSet::new();
    seen.insert(start);

    let mut result = start;

    while let Some(current) = queue.pop_front() {
        // The last node visited will be the furthest.
        result = current;

        let n = map.get(current).unwrap();

        for next in n  {
            if !seen.contains(next.as_str()) {
                queue.push_back(next);
                seen.insert(next);
            }
        }
    }

    result
}
 
fn process(start: &str, end: &str, map: &HashMap<String, Vec<String>>) -> usize {
        
    // get all edges
    let mut edges: IndexSet<Edge> = IndexSet::new();
    for (comp, comp_edges) in map {
        for e in comp_edges {
            edges.insert(Edge {a: comp, b: e});
        }
    }
    
    let mut used: HashSet<Edge> = HashSet::new();
    
    // this will keep the number of nodes visited, which is our result in the 4th iteration
    let mut result = 0;

    // As the minimum cut is 3, the 4th iteration will only be able to reach nodes first sub-graph
    for _ in 0..4 {
        let mut queue = VecDeque::new();
        let mut path = Vec::new();

        queue.push_back((start, usize::MAX));
        result = 0;

        let mut seen: HashSet<&str> = HashSet::new();
        seen.insert(start);

        while let Some((current, head)) = queue.pop_front() {
            result += 1;

            // if we reach the end, then backtrack the path and store it in the used HashSet of 
            // edges so it is used only once
            if current == end {
                let mut index = head;

                // Traverse the linked list.
                while index != usize::MAX {
                    let (edge, next) = path[index];
                    used.insert(edge);
                    index = next;
                }

                break;
            }

            let n = map.get(current).unwrap();

            for next in n {
                let edge = Edge {a: current, b: next};
                if !used.contains(&edge) && !seen.contains(next.as_str()) {
                    queue.push_back((next, path.len()));
                    seen.insert(next);
                    path.push((edge, head));
                }
            }
        }
    }

    result
}

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for line in lines {
            if let Ok(text) = line {
                let mut parts = text.split(":");
                let comp = parts.next().unwrap().trim();
                let comp_links = parts.next().unwrap().trim();
                
                let links_parts = comp_links.split(" ");

                for l in links_parts {
                    map.entry(comp.to_string()).or_default().push(l.to_string());
                    map.entry(l.to_string()).or_default().push(comp.to_string());
                }
            }
        }

        let res = split_components(&map);

        println!("[Part 1] Product of disconnected groups : {}", res);
        
    } else {
        eprintln!("Could not the hailstones from {}", INPUT_FILE);
    }

}
