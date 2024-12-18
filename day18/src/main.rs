use std::iter::repeat_n;

use aoc_derive::aoc_main;
use derive_more::derive::{Deref, DerefMut, From};
use graphs::{UnweightedGraph, bfs};
use grid::Grid;
use math::Vec2D;
use utils::*;

#[derive(Debug, Clone, Deref, DerefMut, From)]
struct Map(Grid<char>);

impl UnweightedGraph for Map {
    type Node = Vec2D;

    fn neighbors<'a, 'b: 'a>(&'a self, node: &'b Vec2D) -> impl Iterator<Item = Vec2D> + 'a {
        self.orthogonal_neighbors(node).filter(|n| self.get(*n) == Some(&'.'))
    }
}

#[aoc_main(70, 1024)]
fn solve(input: Input, size: usize, num_bytes: usize) -> impl Into<Solution> {
    let mut map = Map::from(repeat_n(repeat_n('.', size + 1), size + 1).collect::<Grid<char>>());
    let mut bytes = input.lines().map(extract_two_numbers);

    let (start, end) = ((0, 0), (size, size));

    let part1 = {
        let mut map = map.clone();
        for byte in bytes.clone().take(num_bytes) {
            map[byte] = '#';
        }

        bfs(&map, start, end).distance.unwrap()
    };

    let part2 = bytes
        .find(|&byte| {
            map[byte] = '#';
            bfs(&map, start, end).distance.is_none()
        })
        .unwrap();

    (part1, format!("{},{}", part2.x, part2.y))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        assert_eq!(
            solve(
                r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#
                    .into(),
                6,
                12
            )
            .into(),
            Solution::from((22, "6,1".to_string()))
        );
    }
}
