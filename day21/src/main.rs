use std::{
    collections::{BTreeMap, HashMap},
    iter::{once, repeat, repeat_n},
};

use aoc_derive::aoc_main;
use graphs::{WeightedGraph, dijkstra};
use itertools::{Itertools, iproduct};
use math::Vec2D;
use utils::*;

#[derive(Debug, Clone)]
struct KeypadRobot {
    pos: Vec2D,
    num_dirpads: usize,
}

fn check_path(start: Vec2D, path: &[char], forbidden: Vec2D) -> bool {
    path.iter()
        .try_fold(start, |pos, c| {
            (pos != forbidden).then_some(
                pos + Vec2D::from(match c {
                    '<' => (-1, 0),
                    '>' => (1, 0),
                    '^' => (0, -1),
                    'v' => (0, 1),
                    _ => unreachable!(),
                }),
            )
        })
        .is_some()
}

impl KeypadRobot {
    fn new(num_dirpads: usize) -> Self {
        Self { pos: Self::coord('A'), num_dirpads }
    }

    fn coord(c: char) -> Vec2D {
        match c {
            '7' => (0, 0),
            '8' => (1, 0),
            '9' => (2, 0),
            '4' => (0, 1),
            '5' => (1, 1),
            '6' => (2, 1),
            '1' => (0, 2),
            '2' => (1, 2),
            '3' => (2, 2),
            '0' => (1, 3),
            'A' => (2, 3),
            _ => unreachable!(),
        }
        .into()
    }

    fn expand_move(&self, from: char, to: char) -> usize {
        (0..self.num_dirpads)
            .fold([(make_move(from, to), 1)].into_iter().collect(), |path, _| expand_path(path))
            .into_iter()
            .map(|(path, count)| path.len() * count)
            .sum()
    }

    fn enter_char(&mut self, c: char) -> usize {
        let dist =
            dijkstra(self, [(self.pos, 'A', false)], |&node| node == (Self::coord(c), 'A', true));

        self.pos = Self::coord(c);
        dist.unwrap()
    }

    fn enter_code(&mut self, code: &str) -> usize {
        code.chars().map(|c| self.enter_char(c)).sum()
    }
}

impl WeightedGraph for KeypadRobot {
    type Node = (Vec2D, char, bool);

    fn neighbors<'a, 'b: 'a>(
        &'a self,
        &(pos, keypad_pos, has_pushed): &'b Self::Node,
    ) -> impl Iterator<Item = (Self::Node, graphs::Cost)> + 'a {
        if has_pushed {
            vec![].into_iter()
        } else {
            [
                Some(((pos, 'A', true), self.expand_move(keypad_pos, 'A'))),
                (pos.x < 2)
                    .then_some(((pos + (1, 0), '>', false), self.expand_move(keypad_pos, '>'))),
                (pos.x > 0 && pos != (1, 3))
                    .then_some(((pos + (-1, 0), '<', false), self.expand_move(keypad_pos, '<'))),
                (pos.y < 3 && pos != (0, 2))
                    .then_some(((pos + (0, 1), 'v', false), self.expand_move(keypad_pos, 'v'))),
                (pos.y > 0)
                    .then_some(((pos + (0, -1), '^', false), self.expand_move(keypad_pos, '^'))),
            ]
            .into_iter()
            .flatten()
            .collect_vec()
            .into_iter()
        }
    }
}

fn shortest_sequence(code: &str, num_dirpads: usize) -> usize {
    KeypadRobot::new(num_dirpads).enter_code(code)
}

fn complexity(code: &str, num_dirpads: usize) -> usize {
    println!("check code {code}");
    code[..3].parse::<usize>().unwrap() * shortest_sequence(code, num_dirpads)
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    (
        input.lines().map(|code| complexity(code, 1)).sum_usize(),
        input.lines().map(|code| complexity(code, 24)).sum_usize(),
    )
}

// 454307058698260
// 1153311057543380

fn make_move(from: char, to: char) -> String {
    match (from, to) {
        (from, to) if from == to => "",

        ('A', '>') => "v",
        ('A', '<') => "v<<",
        ('A', '^') => "<",
        ('A', 'v') => "v<",

        ('<', '>') => ">>",
        ('<', 'v') => ">",
        ('<', '^') => ">^",
        ('<', 'A') => ">>^",

        ('^', 'A') => ">",
        ('^', '>') => ">v",
        ('^', 'v') => "v",
        ('^', '<') => "v<",

        ('>', 'A') => "^",
        ('>', '^') => "<^",
        ('>', 'v') => "<",
        ('>', '<') => "<<",

        ('v', 'A') => ">^",
        ('v', '>') => ">",
        ('v', '^') => "^",
        ('v', '<') => "<",

        _ => unreachable!("{from} -> {to}"),
    }
    .to_string()
        + "A"
}

fn expand_move(m: &str) -> Vec<String> {
    ("A".to_string() + m).chars().tuple_windows().map(|(from, to)| make_move(from, to)).collect()
}

fn expand_path(path: BTreeMap<String, usize>) -> BTreeMap<String, usize> {
    path.into_iter()
        .flat_map(|(m, count)| expand_move(&m).into_iter().map(move |m| (m, count)))
        .fold(BTreeMap::new(), |mut acc, (m, count)| {
            *acc.entry(m).or_insert(0) += count;
            acc
        })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    #[test]
    fn test_examples() {
        dbg!(shortest_sequence("0", 2));
        dbg!(shortest_sequence("2", 2));
        dbg!(shortest_sequence("3", 2));
        dbg!(shortest_sequence("6", 2));
        dbg!(shortest_sequence("8", 2));

        dbg!(make_move('A', '<'));
        dbg!(expand_path([(make_move('A', '<'), 1)].into_iter().collect()));

        dbg!(make_move('<', 'A'));
        dbg!(expand_path([(make_move('<', 'A'), 1)].into_iter().collect()));

        //dbg!(expand_path(BTreeMap::from([("<A".to_string(), 2)])));
        //
        //dbg!(expand_path(BTreeMap::from([("<A".to_string(), 1), (">A".to_string(), 1)])));
        //
        //dbg!(expand_path(BTreeMap::from([("<A".to_string(), 2), (">A".to_string(), 1)])));

        dbg!(expand_path(BTreeMap::from([("<A".to_string(), 1)])));
        dbg!(expand_path(expand_path(BTreeMap::from([("<A".to_string(), 1)]))));
        dbg!(expand_path(expand_path(expand_path(BTreeMap::from([("<A".to_string(), 1)])))));

        //return;

        //assert_eq!(shortest_sequence("0", 0), 1);
        //assert_eq!(shortest_sequence("0", 1), 6);
        //// <A
        //assert_eq!(shortest_sequence("0", 0), 6);
        //assert_eq!(shortest_sequence("0", 1), 8);
        //
        //return;

        //assert_eq!(shortest_sequence("029A", 0), 8);
        //assert_eq!(shortest_sequence("029A", 1), 6 + 2 + 2 + 2 + 2 + 4 + 4 + 4);
        assert_eq!(shortest_sequence("029A", 1), 68);
        //        assert_eq!(shortest_sequence("980A".to_string(), 0, 3, &mut HashMap::new()), 60);
        //        assert_eq!(shortest_sequence("179A".to_string(), 0, 3, &mut HashMap::new()), 68);
        //        assert_eq!(shortest_sequence("456A".to_string(), 0, 3, &mut HashMap::new()), 64);
        //        assert_eq!(shortest_sequence("379A".to_string(), 0, 3, &mut HashMap::new()), 64);
        //
        assert_example!(
            "
029A
980A
179A
456A
379A",
            126384
        );
    }
}
