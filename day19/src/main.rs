use std::collections::HashMap;

use aoc_derive::aoc_main;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use utils::*;

fn part1<'a>(patterns: &[&'a str], target: &'a str) -> bool {
    patterns.iter().any(|pattern| match target.strip_prefix(pattern) {
        Some("") => true,
        Some(target) => part1(patterns, target),
        None => false,
    })
}

fn part2<'a>(patterns: &[&'a str], target: &'a str, cache: &mut HashMap<&'a str, usize>) -> usize {
    cache.get(&target).copied().unwrap_or_else(|| {
        let res = patterns
            .iter()
            .map(|&pattern| match target.strip_prefix(pattern) {
                Some("") => 1,
                Some(target) => part2(patterns, target, cache),
                None => 0,
            })
            .sum();
        cache.insert(target, res);
        res
    })
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let (patterns, designs) = input.blocks().collect_tuple().unwrap();

    let patterns = patterns.split(", ").collect_vec();
    let designs = designs.lines().collect_vec();

    let part1 = designs.iter().filter(|design| part1(&patterns, design)).count();

    let part2 = designs
        .par_iter()
        .map(|design| part2(&patterns, design, &mut HashMap::new()))
        .sum::<usize>();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#,
            6,
            16
        );
    }
}
