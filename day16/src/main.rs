use std::collections::{HashMap, HashSet};

use aoc_derive::aoc_main;
use graphs::{WeightedGraph, dijkstra};
use grid::Grid;
use itertools::Itertools;
use math::Vec2D;
use utils::*;

#[derive(Debug, Clone, derive_more::Into, derive_more::Deref)]
struct Maze(Grid<char>);

impl WeightedGraph for Maze {
    type Node = (Vec2D, Vec2D);

    fn neighbors<'a, 'b: 'a>(
        &'a self,
        (pos, heading): &'b Self::Node,
    ) -> impl Iterator<Item = (Self::Node, graphs::Cost)> + 'a {
        self.orthogonal_neighbors(pos).filter_map(|neighbor| {
            (self[neighbor] != '#').then_some((
                (neighbor, *pos - neighbor),
                1 + if *pos - neighbor == *heading { 0 } else { 1000 },
            ))
        })
    }
}

fn all_paths(
    maze: &Maze,
    pos: Vec2D,
    heading: Vec2D,
    score: usize,
    max_score: usize,
    mut path: HashSet<Vec2D>,
    visited: &mut HashMap<(Vec2D, Vec2D), usize>,
) -> HashSet<Vec2D> {
    // This is the main optimization that makes DFS work here:
    // If we already visited this node with a lower score, stop the search
    if let Some(&prev_score) = visited.get(&(pos, heading)) {
        if prev_score < score {
            return HashSet::new();
        }
    }
    visited.insert((pos, heading), score);

    // Stop if we already have a higher cost than the optimal solution
    if score > max_score {
        return HashSet::new();
    }

    path.insert(pos);
    if maze[pos] == 'E' {
        return path;
    }

    maze.neighbors(&(pos, heading))
        // Don't go backwards
        .filter(|((pos, _), _)| !path.contains(pos))
        // Sort ascending, so that we try to go forward before doing a turn
        .sorted_by_key(|((_, _), new_score)| *new_score)
        .flat_map(|((neighbor, new_heading), new_score)| {
            all_paths(
                maze,
                neighbor,
                new_heading,
                score + new_score,
                max_score,
                path.clone(),
                visited,
            )
        })
        .collect()
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let maze = Maze(input.char_grid());

    let (start, heading) = (maze.find_position(&'S').unwrap(), Vec2D::new(1, 0));

    let part1 = dijkstra(&maze, [(start, heading)], |(pos, _)| maze[*pos] == 'E').unwrap();

    let part2 =
        all_paths(&maze, start, heading, 0, part1, HashSet::new(), &mut HashMap::new()).len();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
                "#,
            7036,
            45
        );

        assert_example!(
            "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#^#
#.#.#.#...#...#^#
#.#.#.#.###.#.#^#
#>>v#.#.#.....#^#
#^#v#.#.#.#####^#
#^#v..#.#.#>>>>^#
#^#v#####.#^###.#
#^#v#..>>>>^#...#
#^#v###^#####.###
#^#v#>>^#.....#.#
#^#v#^#####.###.#
#^#v#^........#.#
#^#v#^#########.#
#S#>>^..........#
#################",
            11048,
            64
        );
    }
}
