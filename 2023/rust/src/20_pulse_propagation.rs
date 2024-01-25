mod utils;

use std::collections::{ HashMap, VecDeque };
use std::hash::Hash;

const INPUT_FILE: &str = "./input/20_input.txt";
// const INPUT_FILE: &str = "./input/20_input_test.txt";
// const INPUT_FILE: &str = "./input/20_input_test2.txt";

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
enum Type {
    FlipFlop,
    Conjunction,
    Broadcast
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Module {
    name: String,
    m_type: Type,
    targets: Vec<String>
}

fn parse_module(text: &str) -> Module {
    let mut s1 = text.split("->");
    let mut name = s1.next().unwrap().trim();

    let m_type = match &name[0..1] {
        "%" => Type::FlipFlop,
        "&" => Type::Conjunction,
        _ => Type::Broadcast
    };

    if m_type != Type::Broadcast {
        name = &name[1..];
    }

    let targets = s1.next().unwrap().trim();
    let targets: Vec<String> = targets.split(",").map(|ts| ts.trim().to_string()).collect();

    Module {name: name.trim().to_string(), m_type, targets }
}

fn pulse_module(fm: &Module, tm: &Module, p: bool, states: &mut HashMap<String, bool>) -> bool {
    match tm.m_type {
        Type::FlipFlop if !p => {
            let mut key = tm.name.clone();
            key.push_str("_FF");
            states.entry(key).and_modify(|v| *v = !*v).or_insert(true);
            true
        },
        Type::Conjunction => {
            let mut key = tm.name.clone();
            key.push('#');
            key.push_str(fm.name.as_str());
            states.insert(key, p);
            true
        },
        _ => false
    }
}

fn broadcast(modules: &HashMap<String, Module>, states: &mut HashMap<String, bool>, rx_conj_op: Option<&str>) -> (u32, u32, Vec<String>) {
    let broadcaster = &modules["broadcaster"];

    let mut seen = vec![];

    let mut queue = VecDeque::new();
    queue.push_back(broadcaster);

    let mut high = 0;
    let mut low = 1;

    while let Some(m) = queue.pop_front() {
        match m.m_type {
            Type::Broadcast => {
                for tmn in &m.targets {
                    let tm = &modules[tmn];
                    
                    // pulse target
                    let res = pulse_module(m, tm, false, states);
                    if res {
                        queue.push_back(tm);
                    }

                    low += 1;
                }
            },
            Type::FlipFlop => {
                let mut key = m.name.clone();
                key.push_str("_FF");
                let curr = states[&key];

                for tmn in &m.targets {
                    if let Some(tm) = modules.get(tmn) {
                        // pulse target
                        let res = pulse_module(m, tm, curr, states);
                        if res {
                            queue.push_back(tm);
                        }
                    }

                    if curr == false {
                        low += 1;
                    } else {
                        high += 1;
                    }
                }
            },
            Type::Conjunction => {
                // check if all conjunction entries are HIGH
                let conj_inputs = states.iter().filter(|(k, _)| {
                    let mut key = m.name.clone();
                    key.push('#');
                    k.starts_with(&key)
                });

                let all_high = conj_inputs.clone().all(|(_, p)| *p == true);

                if let Some(rx_conj) = rx_conj_op {
                    if rx_conj == m.name { // we are in the rx conjunction module (part 2)
                        for (mi, p) in conj_inputs {
                            if *p == true {
                                if !seen.contains(mi) {
                                    seen.push(mi.clone());
                                }
                            }
                        }
                    }
                }

                for tmn in &m.targets {
                    if let Some(tm) = modules.get(tmn) {
                        // pulse target
                        let res = pulse_module(m, tm, !all_high, states);
                        if res {
                            queue.push_back(tm);
                        }
                    }

                    if all_high {
                        low += 1;
                    } else {
                        high += 1;
                    }
                }
            }
        }
    }

    (high, low, seen)
}

fn count_pulses(modules: &mut HashMap<String, Module>) -> u32 {
    let mut states: HashMap<String, bool> = HashMap::new();
    
    init_modules_state(modules, &mut states);
    
    let mut high = 0;
    let mut low = 0;
    
    for _ in 0..1000 {
        let (m_high, m_low, _) = broadcast(modules, &mut states, None);
        
        high += m_high;
        low += m_low;
    }
    
    low * high
}

fn count_rx_active_low(modules: &mut HashMap<String, Module>) -> usize {
    let rx_conj = find_conjunction_module_for_rx(modules);

    let mut rx_input_cycles: HashMap<String, u32> = HashMap::new();
    
    let mut states: HashMap<String, bool> = HashMap::new();
    init_modules_state(modules, &mut states);
    
    let rx_input_count = states.keys().filter(|k| {
        let mut kk = rx_conj.name.clone();
        kk.push('#');
        k.starts_with(&kk)
    }).count();
    
    let mut count = 0;
    let mut rx_input_cycles_count = 0;
    
    while rx_input_cycles_count < rx_input_count {
        let (_, _, seen_cycle) = broadcast(modules, &mut states, Some(&rx_conj.name));
        count += 1;

        for sc in seen_cycle {
            if !rx_input_cycles.contains_key(&sc) {
                rx_input_cycles.insert(sc.clone(), count);
                rx_input_cycles_count += 1;
            }
        }
    }

    let nums: Vec<usize> = rx_input_cycles.values().map(|v|*v as usize).collect();
    
    utils::lcm(&nums)
}

fn find_conjunction_module_for_rx(modules: &HashMap<String, Module>) -> &Module {
    let cm = modules.values().find(|m| m.targets.contains(&"rx".to_string())).unwrap();
    assert!(cm.m_type == Type::Conjunction);

    cm
}

fn init_modules_state(modules: &HashMap<String, Module>, states: &mut HashMap<String, bool>) {
    for (_, m) in modules {
        match m.m_type {
            Type::FlipFlop => {
                let mut key = m.name.clone();
                key.push_str("_FF");
                states.insert(key, false);
            },
            Type::Conjunction => {
                for (_, mm) in modules {
                    if mm.targets.contains(&m.name) {
                        let mut key = m.name.clone();
                        key.push('#');
                        key.push_str(mm.name.clone().as_str());
                        states.insert(key, false);
                    }
                }
            },
            _ => {}
        }
    }
}

fn _print_modules_state(states: &mut HashMap<String, bool>) {
    println!("===========================");
    for (m, s) in states {
        println!("{m} = {s}");
    }
}

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut modules: HashMap<String, Module> = HashMap::new();

        for line in lines {
            if let Ok(text) = line {
                let module = parse_module(&text);
                modules.insert(module.name.clone(), module);
            }
        }
        
        let sum1 = count_pulses(&mut modules);
        println!("[Part 1] Multiplication of low pulses with high pulses : {}", sum1);

        let sum2 = count_rx_active_low(&mut modules);
        println!("[Part 2] Fewest number of button presses required for rx : {}", sum2);
    } else {
        eprintln!("Could not load module communication from {}", INPUT_FILE);
    }

}
