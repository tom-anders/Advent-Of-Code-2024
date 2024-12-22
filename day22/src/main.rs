use std::collections::{HashMap, HashSet};

use aoc_derive::aoc_main;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use utils::*;

fn iterate(mut n: i64) -> i64 {
    n ^= n * 64;
    n %= 16777216;
    n ^= n / 32;
    n %= 16777216;
    n ^= n * 2048;
    n %= 16777216;
    n
}

fn secret_numbers(n: i64) -> Vec<i64> {
    (0..2000).fold(vec![n], |mut numbers, _| {
        numbers.push(iterate(*numbers.last().unwrap()));
        numbers
    })
}

#[derive(Debug)]
struct Buyer {
    numbers: Vec<i64>,
    diffs: HashMap<Vec<i64>, i64>,
}

impl Buyer {
    fn new(secret: i64) -> Self {
        let numbers = secret_numbers(secret);
        let prices = numbers.iter().map(|n| n % 10).collect_vec();

        let diffs = prices.iter().tuple_windows().map(|(a, b)| b - a).collect_vec();

        let diffs = diffs.windows(4).enumerate().fold(HashMap::new(), |mut map, (i, window)| {
            map.entry(window.into()).or_insert(prices[i + 4]);
            map
        });

        Self { numbers, diffs }
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let buyers = input.lines().map(|line| Buyer::new(line.parse().unwrap())).collect_vec();

    let part1 = buyers.iter().map(|b| *b.numbers.last().unwrap()).sum_i64();

    let part2 = buyers
        .iter()
        .flat_map(|b| b.diffs.clone().into_keys())
        .collect::<HashSet<_>>()
        .into_par_iter()
        .map(|diffs| buyers.iter().map(|b| *b.diffs.get(&diffs).unwrap_or(&0)).sum_i64())
        .max()
        .unwrap();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;

        assert_example!(
            r#"1
10
100
2024"#,
            37327623
        );

        assert_part2!(
            r#"1
2
3
2024"#,
            23
        );
    }
}
