use std::{collections::HashMap, ops::ControlFlow};

use aoc_derive::aoc_main;
use grid::Grid;
use itertools::Itertools;
use math::Vec2D;
use utils::*;

fn antinodes_part2(a: Vec2D, b: Vec2D, grid: &Grid<char>) -> impl Iterator<Item = Vec2D> {
    let mut antinodes = vec![];
    let mut pos = a;
    while grid.contains(&pos) {
        antinodes.push(pos);
        pos += a - b;
    }
    antinodes.into_iter()
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let grid = input.char_grid();

    let antenna_map = grid.iter().filter_map(|(pos, &c)| (c != '.').then_some((c, pos))).fold(
        HashMap::<_, Vec<_>>::new(),
        |mut map, (c, pos)| {
            map.entry(c).or_default().push(pos);
            map
        },
    );

    let part1 = antenna_map
        .iter()
        .flat_map(|(_, antennas)| {
            antennas
                .clone()
                .into_iter()
                .combinations(2)
                .flat_map(|pair| [pair[0] + (pair[0] - pair[1]), pair[1] + (pair[1] - pair[0])])
                .filter(|antinode| grid.contains(antinode))
        })
        .unique()
        .count();

    let part2 = antenna_map
        .into_iter()
        .flat_map(|(_, antennas)| {
            antennas.clone().into_iter().combinations(2).flat_map(|pair| {
                antinodes_part2(pair[0], pair[1], &grid)
                    .chain(antinodes_part2(pair[1], pair[0], &grid))
            })
        })
        .unique()
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
            r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............

                "#,
            14,
            34
        );
    }
}
