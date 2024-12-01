use std::collections::HashMap;

use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::*;

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let mut left = Vec::<usize>::new();
    let mut right = Vec::<usize>::new();

    for line in input.lines() {
        let (l, r) = extract_numbers(line).collect_tuple().unwrap();
        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    let part1 = left.iter().zip(right.iter()).map(|(l, r)| l.abs_diff(*r)).sum_usize();

    let right_counts = right.into_iter().fold(HashMap::new(), |mut map, n| {
        *map.entry(n).or_default() += 1;
        map
    });

    let part2 = left.into_iter().map(|n| n * right_counts.get(&n).unwrap_or(&0)).sum_usize();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"3   4
4   3
2   5
1   3
3   9
3   3
                "#,
            11,
            31
        );
    }
}
