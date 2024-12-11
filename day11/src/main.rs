use std::collections::HashMap;

use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::*;

fn iterate(stone: usize, depth: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if depth == 0 {
        return 1;
    }

    if let Some(cached) = cache.get(&(stone, depth)) {
        *cached
    } else {
        let count = if stone == 0 {
            iterate(1, depth - 1, cache)
        } else {
            let s = stone.to_string();
            if s.len() % 2 == 0 {
                iterate(s[s.len() / 2..].parse().unwrap(), depth - 1, cache)
                    + iterate(s[..s.len() / 2].parse().unwrap(), depth - 1, cache)
            } else {
                iterate(stone * 2024, depth - 1, cache)
            }
        };
        cache.insert((stone, depth), count);
        count
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let stones = input.as_str().split(' ').flat_map(str::parse).collect_vec();

    let mut cache = HashMap::new();
    (
        stones.iter().map(|stone| iterate(*stone, 25, &mut cache)).sum_usize(),
        stones.iter().map(|stone| iterate(*stone, 75, &mut cache)).sum_usize(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!("125 17", 55312);
    }
}
