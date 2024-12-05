use std::str::FromStr;

use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::*;

#[derive(parse_display::FromStr)]
#[display("{first}|{second}")]
struct Rule {
    first: usize,
    second: usize,
}

#[derive(aoc_derive::CollectFromStr, Clone, derive_more::Deref, derive_more::DerefMut)]
struct Pages(Vec<usize>);

impl Pages {
    fn score(&self) -> usize {
        self.0[self.0.len() / 2]
    }

    fn correct(&self, rules: &[Rule]) -> bool {
        rules.iter().all(|rule| (0..self.0.len()).all(|index| self.check_rule(index, rule)))
    }

    fn check_rule(&self, index: usize, rule: &Rule) -> bool {
        if self.0[index] == rule.first {
            let right_index =
                self.0.iter().enumerate().find_map(|(j, p)| (*p == rule.second).then_some(j));

            if right_index.is_some_and(|right_index| right_index < index) {
                return false;
            }
        }
        true
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let (rules, pages) = input.blocks().collect_tuple().unwrap();

    let rules = rules.lines().map(|line| Rule::from_str(line).unwrap()).collect_vec();

    let pages = pages.lines().map(|line| line.parse::<Pages>().unwrap()).collect_vec();

    let (correct, mut incorrect): (Vec<_>, Vec<_>) =
        pages.into_iter().partition(|pages| pages.correct(&rules));

    let part1 = correct.iter().map(Pages::score).sum_usize();

    for incorrect_pages in &mut incorrect {
        while !incorrect_pages.correct(&rules) {
            for rule in &rules {
                for (i, page) in incorrect_pages.clone().iter().enumerate() {
                    if *page == rule.first {
                        let j = incorrect_pages
                            .iter()
                            .enumerate()
                            .find_map(|(j, p)| (*p == rule.second).then_some(j));
                        if let Some(j) = j {
                            if j < i {
                                incorrect_pages.swap(i, j);
                            }
                        }
                    }
                }
            }
        }
    }

    let part2 = incorrect.iter().map(Pages::score).sum_usize();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
                "#,
            143,
            123
        );
    }
}
