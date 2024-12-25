#![feature(let_chains)]

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::BufWriter,
};

use aoc_derive::aoc_main;
use itertools::{Itertools, iproduct};
use utils::*;

#[derive(Debug, Clone, Copy, parse_display::Display, parse_display::FromStr)]
#[allow(non_snake_case, clippy::upper_case_acronyms)]
enum GateType {
    AND,
    XOR,
    OR,
}

#[derive(Debug, Clone)]
struct Gate<'a> {
    lhs: &'a str,
    rhs: &'a str,
    ty: GateType,
    output: &'a str,
}

#[derive(Debug, Clone)]
struct System<'a> {
    values: HashMap<&'a str, bool>,
    gates: Vec<Gate<'a>>,
}

impl System<'_> {
    fn part1(mut self) -> Option<usize> {
        let z_names = self
            .values
            .keys()
            .copied()
            .chain(self.gates.iter().flat_map(|gate| vec![gate.lhs, gate.rhs, gate.output]))
            .filter(|name| name.starts_with('z'))
            .collect::<HashSet<_>>();

        let mut prev_len = self.values.len();
        while !z_names.iter().all(|name| self.values.contains_key(name)) {
            for gate in &self.gates {
                if let Some(&lhs) = self.values.get(gate.lhs)
                    && let Some(&rhs) = self.values.get(gate.rhs)
                {
                    match gate.ty {
                        GateType::AND => {
                            self.values.insert(gate.output, lhs && rhs);
                        }
                        GateType::XOR => {
                            self.values.insert(gate.output, lhs ^ rhs);
                        }
                        GateType::OR => {
                            self.values.insert(gate.output, lhs || rhs);
                        }
                    }
                }
            }

            if prev_len == self.values.len() {
                return None;
            }
            prev_len = self.values.len();
        }

        Some(
            z_names
                .iter()
                .map(|z| {
                    let n: u32 = z[1..].parse().unwrap();
                    if self.values[z] { 2_usize.pow(n) } else { 0 }
                })
                .sum_usize(),
        )
    }

    fn val(&self, var: &str) -> usize {
        self.values
            .keys()
            .copied()
            .filter(|v| v.starts_with(var))
            .map(|v| {
                let n: u32 = v[1..].parse().unwrap();
                if self.values[v] { 2_usize.pow(n) } else { 0 }
            })
            .sum_usize()
    }

    fn clone_with_swapped(&self, lhs: &str, rhs: &str) -> Self {
        let mut res = self.clone();

        let lhs = res
            .gates
            .iter()
            .enumerate()
            .find_map(|(index, gate)| (gate.output == lhs).then_some(index))
            .unwrap();

        let rhs = res
            .gates
            .iter()
            .enumerate()
            .find_map(|(index, gate)| (gate.output == rhs).then_some(index))
            .unwrap();

        unsafe { std::ptr::swap(&mut res.gates[lhs].output, &mut res.gates[rhs].output) };
        res
    }

    fn check(self) -> bool {
        let x = self.val("x");
        let y = self.val("y");

        self.part1() == Some(x + y)
    }
}

fn part2(mut system: System) -> String {
    // Found these by just looking at the graph
    system = system
        .clone_with_swapped("z10", "ggn")
        .clone_with_swapped("z32", "grm")
        .clone_with_swapped("z39", "twr");

    let mut file = BufWriter::new(File::create("target/24.dot").unwrap());
    use dot_writer::*;
    let mut writer = DotWriter::from(&mut file);

    let mut graph = writer.digraph();
    graph.graph_attributes().set("nodesep", "2.0", false).set("ranksep", "1.0", false);
    for gate in system.gates.iter().sorted_by_key(|g| g.ty as u8) {
        let color = match gate.ty {
            GateType::AND => Color::Blue,
            GateType::XOR => Color::Red,
            GateType::OR => Color::PaleGreen,
        };
        graph.edge(gate.lhs, gate.output).attributes().set_color(color);
        graph.edge(gate.rhs, gate.output).attributes().set_color(color);

        if gate.output.starts_with("z") {
            graph.node_named(gate.output).set_pen_width(2.0).set_color(Color::PaleTurquoise);
        }
    }

    println!("{:b}", system.clone().part1().unwrap());
    println!("{:b}", system.val("x") + system.val("y"));

    // Find last swap by brute force
    let outputs = system.clone().gates.iter().map(|gate| gate.output).collect_vec();

    let swaps = iproduct!(outputs.iter(), outputs.iter())
        .filter(|(lhs, rhs)| system.clone_with_swapped(lhs, rhs).check())
        .filter(|(lhs, rhs)| {
            (0..=44).all(|x| {
                let mut system = system.clone_with_swapped(lhs, rhs);
                let var = format!("x{x:02}");
                *system.values.get_mut(var.as_str()).unwrap() = !system.values[var.as_str()];
                system.check()
            }) && (0..=44).all(|y| {
                let mut system = system.clone_with_swapped(lhs, rhs);
                let var = format!("y{y:02}");
                *system.values.get_mut(var.as_str()).unwrap() = !system.values[var.as_str()];
                system.check()
            })
        })
        .collect_vec();

    println!("Found swaps: {}", swaps.len());

    let (lhs, rhs) = swaps.last().unwrap();

    println!("Last swap is {lhs} {rhs}");

    system = system.clone_with_swapped(lhs, rhs);

    println!("{:b}", system.clone().part1().unwrap());
    println!("{:b}", system.val("x") + system.val("y"));

    ["z10", "ggn", "z32", "grm", "z39", "twr", *lhs, *rhs].into_iter().sorted().join(",")
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let (values, gates) = input.blocks().collect_tuple().unwrap();

    let values: HashMap<&str, bool> = values
        .lines()
        .map(|line| {
            let (name, val) = line.split_once(": ").unwrap();
            (name, val == "1")
        })
        .collect();

    let gates = gates
        .lines()
        .map(|line| {
            let (lhs, ty, rhs, _, output) = line.split(" ").collect_tuple().unwrap();
            Gate { lhs, rhs, ty: ty.parse().unwrap(), output }
        })
        .collect_vec();

    let system = System { values, gates };

    // idea: check which bits are already correct.
    // then try swapping only outputs that affect the already correct ones

    (system.clone().part1(), part2(system))
}
