use std::{collections::BTreeMap, iter::once};

use aoc_derive::aoc_main;
use graphs::{WeightedGraph, dijkstra};
use itertools::Itertools;
use math::Vec2D;
use utils::*;

// Order matters here! e.g., changing >^ to ^> changes the shortest path to the code.
// One could write an algorithm that tries all legal move orders and the permutations for each
// path, but lucklily they all seem to be independent, so I manually tried all legal move orders
// for each path, keeping the one that yielded the lowest answer in the end.
fn move_on_dirpad(from: char, to: char) -> String {
    match (from, to) {
        (from, to) if from == to => "",

        ('A', '>') => "v",
        ('A', '<') => "v<<",
        ('A', '^') => "<",
        ('A', 'v') => "<v",

        ('<', '>') => ">>",
        ('<', 'v') => ">",
        ('<', '^') => ">^",
        ('<', 'A') => ">>^",

        ('^', 'A') => ">",
        ('^', '>') => "v>",
        ('^', 'v') => "v",
        ('^', '<') => "v<",

        ('>', 'A') => "^",
        ('>', '^') => "<^",
        ('>', 'v') => "<",
        ('>', '<') => "<<",

        ('v', 'A') => "^>",
        ('v', '>') => ">",
        ('v', '^') => "^",
        ('v', '<') => "<",

        _ => unreachable!("{from} -> {to}"),
    }
    .to_string()
        + "A"
}

fn expand_moves(moves: &str) -> Vec<String> {
    ("A".to_string() + moves)
        .chars()
        .tuple_windows()
        .map(|(from, to)| move_on_dirpad(from, to))
        .collect()
}

fn expand_path(path: BTreeMap<String, usize>) -> BTreeMap<String, usize> {
    path.into_iter()
        .flat_map(|(m, count)| expand_moves(&m).into_iter().map(move |m| (m, count)))
        .fold(BTreeMap::new(), |mut acc, (m, count)| {
            *acc.entry(m).or_insert(0) += count;
            acc
        })
}

#[derive(Debug, Clone)]
struct KeypadRobot {
    num_dirpads: usize,
}

impl KeypadRobot {
    fn new(num_dirpads: usize) -> Self {
        Self { num_dirpads }
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
            .fold([(move_on_dirpad(from, to), 1)].into_iter().collect(), |path, _| {
                expand_path(path)
            })
            .into_iter()
            .map(|(path, count)| path.len() * count)
            .sum()
    }

    fn enter_code(&mut self, code: &str) -> usize {
        once('A')
            .chain(code.chars())
            .tuple_windows()
            .map(|(from, to)| {
                dijkstra(self, [(Self::coord(from), 'A', false)], |&node| {
                    node == (Self::coord(to), 'A', true)
                })
                .unwrap()
            })
            .sum()
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

fn complexity(code: &str, num_dirpads: usize) -> usize {
    code[..3].parse::<usize>().unwrap() * KeypadRobot::new(num_dirpads).enter_code(code)
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    (
        input.lines().map(|code| complexity(code, 1)).sum_usize(),
        input.lines().map(|code| complexity(code, 24)).sum_usize(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
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
