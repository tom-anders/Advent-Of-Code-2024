use std::collections::HashSet;

use aoc_derive::aoc_main;
use grid::Grid;
use itertools::Itertools;
use math::Vec2D;
use utils::*;

fn iterate(map: &Grid<char>, start: Vec2D) -> Option<HashSet<(Vec2D, Vec2D)>> {
    let mut pos = start;
    let mut heading = Vec2D::new(0, -1);
    let mut visited = HashSet::new();

    loop {
        if visited.contains(&(pos, heading)) {
            return None; // loop detected
        }
        visited.insert((pos, heading));
        match map.get(pos + heading) {
            None => break,
            Some(&'#') => heading = heading.rotated_right(),
            _ => pos += heading,
        }
    }
    Some(visited)
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let mut map = input.char_grid();

    let start = map.iter().find_map(|(pos, &c)| (c == '^').then_some(pos)).unwrap();

    let visited: HashSet<_> =
        iterate(&map, start).unwrap().into_iter().map(|(pos, _)| pos).unique().collect();

    let part1 = visited.len();

    let part2 = visited
        .into_iter()
        .filter(|&pos| {
            map[pos] = '#';
            let res = iterate(&map, start);
            map[pos] = '.';
            res.is_none()
        })
        .count();

    (part1, part2)
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
