use aoc_derive::aoc_main;
use grid::Grid;
use itertools::Itertools;
use math::Vec2D;
use utils::*;

fn hiking_paths(pos: Vec2D, map: &Grid<usize>) -> Vec<Vec2D> {
    if map[pos] == 9 {
        return vec![pos];
    }

    map.orthogonal_neighbors(&pos)
        .filter(|&neighbor| map[neighbor] == map[pos] + 1)
        .flat_map(|pos| hiking_paths(pos, map))
        .collect_vec()
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let map = input.number_grid();

    map.iter().filter_map(|(pos, &val)| (val == 0).then_some(pos)).fold(
        (0, 0),
        |(part1, part2), trailhead| {
            let score = hiking_paths(trailhead, &map);
            (part1 + score.iter().unique().count(), part2 + score.len())
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"0123
1234
8765
9876
                "#,
            1
        );

        assert_example!(
            r#"
            89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
            "#,
            36,
            81
        );
    }
}
