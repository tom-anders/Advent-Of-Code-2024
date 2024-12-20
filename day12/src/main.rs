use std::collections::{HashMap, HashSet};

use aoc_derive::aoc_main;
use graphs::{UnweightedGraph, floodfill};
use math::Vec2D;
use utils::*;

#[derive(Debug, derive_more::From, derive_more::Deref, derive_more::DerefMut)]
struct Map(grid::Grid<char>);

impl UnweightedGraph for Map {
    type Node = Vec2D;

    fn neighbors<'a, 'b: 'a>(&'a self, node: &'b Vec2D) -> impl Iterator<Item = Vec2D> + 'a {
        node.orthogonal_neighbors().filter(|n| self.get(*n) == Some(&self[*node]))
    }
}

fn sides(region: &HashSet<Vec2D>, map: &Map) -> usize {
    let c = map[*region.iter().next().unwrap()];

    let edges = region.iter().filter(|p| p.orthogonal_neighbors().any(|n| map.get(n) != Some(&c)));

    let mut visited = HashSet::new();

    let mut sides = 0;

    for edge in edges {
        if visited.contains(edge) {
            continue;
        }
        visited.insert(*edge);

        let edge_normal = edge
            .orthogonal_neighbors()
            .find_map(|n| (map.get(n) != Some(&c)).then_some(n - *edge))
            .unwrap();

        let mut pos = *edge;
        let mut normal = edge_normal;

        loop {
            if !region.contains(&(pos + normal))
                && !region.contains(&(pos + normal.rotated_right()))
            {
                normal = normal.rotated_right();
                sides += 1;
            } else if region.contains(&(pos + normal.rotated_right()))
                && region.contains(&(pos + normal.rotated_right() + normal))
            {
                pos = pos + normal.rotated_right() + normal;
                normal = normal.rotated_left();
                sides += 1;
            } else {
                pos += normal.rotated_right();
            }

            visited.insert(pos);

            if pos == *edge && normal == edge_normal {
                break;
            }
        }
    }

    sides
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let map = Map::from(input.char_grid());

    let mut region_map = HashMap::<char, Vec<HashSet<Vec2D>>>::new();

    for (pos, c) in map.iter() {
        let regions = region_map.entry(*c).or_default();

        if !regions.iter().any(|r| r.contains(&pos)) {
            regions.push(floodfill(&map, pos).keys().copied().collect())
        }
    }

    let (part1, part2) = region_map
        .iter()
        .flat_map(|(c, regions)| {
            regions.iter().map(|region| {
                let area = region.len();
                let perimeter = region
                    .iter()
                    .map(|pos| {
                        pos.orthogonal_neighbors().filter(|n| map.get(*n) != Some(c)).count()
                    })
                    .sum_usize();
                (area * perimeter, area * sides(region, &map))
            })
        })
        .unzip_vec();

    (part1.into_iter().sum_usize(), part2.into_iter().sum_usize())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"
AAAA
BBCD
BBCC
EEEC"#,
            140,
            80
        );

        assert_example!(
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
            772
        );

        assert_example!(
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
            1930,
            1206
        );

        assert_part2!(
            "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
            236
        );

        assert_part2!(
            "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
            368
        );
    }
}
