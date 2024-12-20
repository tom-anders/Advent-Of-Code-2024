use aoc_derive::aoc_main;
use derive_more::derive::{Deref, DerefMut, From};
use graphs::{UnweightedGraph, bfs, floodfill};
use grid::Grid;
use math::Vec2D;
use utils::*;

#[derive(Debug, Clone, From, Deref, DerefMut)]
struct Racetrack(Grid<char>);

impl UnweightedGraph for Racetrack {
    type Node = Vec2D;

    fn neighbors<'a, 'b: 'a>(&'a self, node: &'b Vec2D) -> impl Iterator<Item = Vec2D> + 'a {
        self.orthogonal_neighbors(node).filter(|&n| self[n] != '#')
    }
}

#[aoc_main(100)]
fn solve(input: Input, min_save: usize) -> impl Into<Solution> {
    let track: Racetrack = input.char_grid().into();
    let start = track.find_position(&'S').unwrap();
    let end = track.find_position(&'E').unwrap();

    let best_without_cheat = bfs(&track, start, end).distance.unwrap();

    let distance_from_start = floodfill(&track, start);
    let distance_from_end = floodfill(&track, end);

    let find_cheats = |pos: Vec2D, max_cheat: usize| {
        track
            .iter()
            .filter_map(|(pos_after_cheat, &c)| {
                let dist = (pos - pos_after_cheat).manhattan_dist();
                (c != '#' && dist <= max_cheat).then_some((pos_after_cheat, dist))
            })
            .filter(|&(pos_after_cheat, dist)| {
                let total_dist =
                    distance_from_start[&pos] + dist + distance_from_end[&pos_after_cheat];
                total_dist < best_without_cheat && best_without_cheat - total_dist >= min_save
            })
            .count()
    };

    (
        track.iter().filter(|&(_, &c)| c != '#').map(|(pos, _)| find_cheats(pos, 2)).sum_usize(),
        track.iter().filter(|&(_, &c)| c != '#').map(|(pos, _)| find_cheats(pos, 20)).sum_usize(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        assert_eq!(
            solve(
                r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#
                    .into(),
                0
            )
            .into()
            .part1,
            Some((14 + 14 + 2 + 4 + 2 + 3 + 5).to_string())
        );

        assert_eq!(
            solve(
                r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#
                    .into(),
                50
            )
            .into()
            .part2,
            Some((32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3).to_string())
        );
    }
}
