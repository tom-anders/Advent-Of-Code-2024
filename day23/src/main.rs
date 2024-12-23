use std::collections::{BTreeSet, HashMap, HashSet};

use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::*;

struct Computer<'a> {
    name: &'a str,
    conns: HashSet<&'a str>,
}

impl<'a> Computer<'a> {
    fn new(name: &'a str) -> Self {
        Self { name, conns: HashSet::new() }
    }
}

struct Network<'a> {
    computers: HashMap<&'a str, Computer<'a>>,
}

impl<'a> Network<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            computers: s.lines().map(|line| line.split_once('-').unwrap()).fold(
                HashMap::new(),
                |mut computers, (lhs, rhs)| {
                    computers.entry(lhs).or_insert_with(|| Computer::new(lhs)).conns.insert(rhs);
                    computers.entry(rhs).or_insert_with(|| Computer::new(rhs)).conns.insert(lhs);
                    computers
                },
            ),
        }
    }

    fn find_triangles(&self) -> usize {
        self.computers
            .values()
            .flat_map(|computer| {
                computer.conns.iter().permutations(2).filter_map(move |pair| {
                    self.computers[pair[0]]
                        .conns
                        .contains(pair[1])
                        .then(|| BTreeSet::from([computer.name, pair[0], pair[1]]))
                })
            })
            .unique()
            .filter(|set| set.iter().any(|name| name.starts_with('t')))
            .count()
    }

    fn find_largest_lan_party_password(&self) -> String {
        self.computers
            .values()
            .map(|computer| {
                computer.conns.iter().fold(
                    BTreeSet::from([computer.name]),
                    |mut party, computer| {
                        if party.iter().all(|c| self.computers[c].conns.contains(computer)) {
                            party.insert(computer);
                        }
                        party
                    },
                )
            })
            .unique()
            .max_by_key(|network| network.len())
            .unwrap()
            .into_iter()
            .sorted()
            .join(",")
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let network = Network::new(input.as_str());

    (network.find_triangles(), network.find_largest_lan_party_password())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#,
            7,
            "co,de,ka,ta"
        );
    }
}
