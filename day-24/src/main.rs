use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use rand::prelude::*;

#[derive(Clone, PartialEq)]
enum ComponentType {
    WIRE,
    XOR,
    AND,
    OR,
}

#[derive(Clone)]
struct Component {
    component_type: ComponentType,
    inputs: Vec<u64>,
}

impl Component {
    fn new(component_type: ComponentType) -> Self {
        Component {
            component_type,
            inputs: vec![],
        }
    }

    fn new_wire(initial_value: u64) -> Self {
        Component {
            component_type: ComponentType::WIRE,
            inputs: vec![initial_value],
        }
    }
}

fn get_graph(input: Vec<String>) -> (Vec<(String, String)>, HashMap<String, Component>) {
    let mut input = input.split(|line| line.is_empty());

    let mut state = HashMap::<String, Component>::new();
    for line in input.next().unwrap() {
        let (id, initial_value) = line.split_once(": ").unwrap();
        let id = id.to_string();
        let initial_value = initial_value.parse::<u64>().unwrap();
        state.insert(id.clone(), Component::new_wire(initial_value));
    }

    let mut adj = Vec::new();
    for (gate_id, line) in input.next().unwrap().into_iter().enumerate() {
        let mut data = line.split(" ");
        let input_1 = data.next().unwrap().to_string();
        let component_type = data.next().unwrap();
        let input_2 = data.next().unwrap().to_string();
        let output = data.skip(1).next().unwrap().to_string();

        if !state.contains_key(&input_1) {
            state.insert(input_1.clone(), Component::new(ComponentType::WIRE));
        }
        if !state.contains_key(&input_2) {
            state.insert(input_2.clone(), Component::new(ComponentType::WIRE));
        }

        let component = match component_type {
            "AND" => Component::new(ComponentType::AND),
            "OR" => Component::new(ComponentType::OR),
            "XOR" => Component::new(ComponentType::XOR),
            _ => unreachable!(),
        };
        let gate_id = format!("GATE{}", gate_id);

        state.insert(gate_id.clone(), component);
        state.insert(output.clone(), Component::new(ComponentType::WIRE));

        adj.push((input_1, gate_id.clone()));
        adj.push((input_2, gate_id.clone()));
        adj.push((gate_id, output));
    }

    (adj, state)
}

fn eval(
    adj: &Vec<(String, String)>,
    mut state: HashMap<String, Component>,
    input: Option<(u64, u64)>,
) -> u64 {
    let mut adj_map = HashMap::<_, Vec<_>>::new();
    let mut in_deg = HashMap::<String, u8>::new();
    for (u, v) in adj {
        adj_map.entry(u.clone()).or_default().push(v.clone());
        in_deg.entry(u.clone()).or_default();
        *in_deg.entry(v.clone()).or_default() += 1;
    }
    let adj = adj_map;

    if input.is_some() {
        let (a, b) = input.unwrap();

        for i in 0..=44 {
            state.get_mut(&format!("x{:0>2}", i)).unwrap().inputs[0] = (a >> i) & 1;
        }

        for i in 0..=44 {
            state.get_mut(&format!("y{:0>2}", i)).unwrap().inputs[0] = (b >> i) & 1;
        }
    }

    let mut q = VecDeque::new();
    for (id, &deg) in &in_deg {
        if deg == 0 {
            q.push_back(id.clone());
        }
    }

    while let Some(id) = q.pop_front() {
        let curr = state.get_mut(&id).unwrap();
        let output = match curr.component_type {
            ComponentType::WIRE => curr.inputs[0],
            ComponentType::AND => curr.inputs[0] & curr.inputs[1],
            ComponentType::OR => curr.inputs[0] | curr.inputs[1],
            ComponentType::XOR => curr.inputs[0] ^ curr.inputs[1],
        };
        for next in adj.get(&id).or(Some(&vec![])).unwrap() {
            state.get_mut(next).unwrap().inputs.push(output);
            let deg = in_deg.get_mut(next).unwrap();
            *deg -= 1;
            if *deg == 0 {
                q.push_back(next.clone());
            }
        }
    }

    state
        .into_iter()
        .filter(|(id, _)| id.starts_with("z"))
        .filter_map(|(id, component)| {
            let id = id[1..].parse::<u8>().unwrap();
            match component.component_type {
                ComponentType::WIRE => component.inputs.get(0).map(|x| x * (1 << id)),
                _ => unreachable!(),
            }
        })
        .sum()
}

fn solve_1(input: Vec<String>) {
    let (adj, state) = get_graph(input);

    println!("{}", eval(&adj, state, None));
}

fn find_rightmost_error_bit(
    adj: &Vec<(String, String)>,
    state: &HashMap<String, Component>,
) -> u32 {
    let mut rng = rand::thread_rng();

    let mut rightmost = u32::MAX;
    for _ in 0..20 {
        let a = rng.gen_range(0..1u64 << 45);
        let b = rng.gen_range(0..1u64 << 45);

        let got = eval(adj, state.clone(), Some((a, b)));
        let actual = a + b;

        let diff = got ^ actual;
        rightmost = rightmost.min(diff.trailing_zeros());
    }

    rightmost
}

fn solve_2(input: Vec<String>) {
    let (mut adj, mut state) = get_graph(input);

    let mut ans = vec![];
    loop {
        let rightmost_error_bit = find_rightmost_error_bit(&adj, &mut state);
        if rightmost_error_bit == 64 {
            break;
        }

        let mut swap_adj = vec![];
        'outer: for e1 in &adj {
            if state.get(&e1.1).unwrap().component_type != ComponentType::WIRE {
                continue;
            }

            for e2 in &adj {
                if state.get(&e2.1).unwrap().component_type != ComponentType::WIRE {
                    continue;
                }

                let mut new_adj = HashSet::new();
                for edge in &adj {
                    if edge != e1 && edge != e2 {
                        new_adj.insert(edge.clone());
                    }
                }

                let new_e1 = (e1.0.clone(), e2.1.clone());
                if new_adj.contains(&new_e1) {
                    continue;
                }
                new_adj.insert(new_e1.clone());

                let new_e2 = (e2.0.clone(), e1.1.clone());
                if new_adj.contains(&new_e2) {
                    continue;
                }
                new_adj.insert(new_e2.clone());

                let new_adj = new_adj.into_iter().collect();
                let new_rightmost_error_bit = find_rightmost_error_bit(&new_adj, &state);
                if new_rightmost_error_bit > rightmost_error_bit {
                    swap_adj = new_adj;
                    ans.push(e1.1.clone());
                    ans.push(e2.1.clone());
                    break 'outer;
                }
            }
        }
        adj = swap_adj;
    }

    ans.sort_unstable();
    let ans = ans.join(",");
    println!("{ans}");
}

fn main() {
    let input: Vec<_> = fs::read_to_string("input.txt")
        .unwrap()
        .split_terminator('\n')
        .map(|x| x.to_string())
        .collect();

    solve_1(input.clone());
    solve_2(input.clone());
}
