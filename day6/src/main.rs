#![feature(hash_set_entry)]

use std::collections::{HashSet, hash_set::Entry};

use aoc_derive::aoc_main;
use grid::Grid;
use math::Vec2D;
use utils::*;

fn part1_visited(map: &Grid<char>, start: Vec2D) -> HashSet<Vec2D> {
    let mut pos = start;

    let mut heading = Vec2D::new(0, -1);

    let mut visited = HashSet::new();
    loop {
        visited.insert(pos);
        match map.get(pos + heading) {
            None => break,
            Some(&'#') => heading = heading.rotated_right(),
            _ => {
                pos += heading;
            }
        }
    }
    visited
}

fn check_obstruction(map: &Grid<char>, mut pos: Vec2D) -> bool {
    let mut visited = HashSet::new();
    let mut heading = Vec2D::new(0, -1);
    visited.insert((pos, heading));
    loop {
        match map.get(pos + heading) {
            None => {
                return false;
            }
            Some(&'#') => {
                heading = heading.rotated_right();
            }
            _ => {
                pos += heading;
            }
        }
        match visited.entry((pos, heading)) {
            Entry::Vacant(entry) => entry.insert(),
            Entry::Occupied(_) => return true,
        }
    }
}

fn part2(mut map: Grid<char>, start: Vec2D, to_check: &HashSet<Vec2D>) -> usize {
    to_check
        .iter()
        .filter(|&&block| {
            map[block] = '#';
            let res = check_obstruction(&map, start);
            map[block] = '.';
            res
        })
        .count()
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let map = input.char_grid();

    let start = map.iter().find_map(|(pos, &c)| (c == '^').then_some(pos)).unwrap();

    let visited = part1_visited(&map, start);

    (visited.len(), part2(map, start, &visited))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
                "#,
            41,
            6
        );
    }
}
