use aoc_derive::aoc_main;
use itertools::Itertools;
use num::{abs, signum};
use utils::*;

fn safe2(levels: &[i64]) -> bool {
    (0..levels.len()).any(|skip| safe(levels.iter().skip_nth(skip)))
}

fn safe<'a>(levels: impl Iterator<Item = &'a i64>) -> bool {
    levels
        .tuple_windows()
        .try_fold(None, |prev_sign, (a, b)| {
            let diff = abs(a - b);
            let sign = signum(a - b);
            if diff > 0 && diff <= 3 && (prev_sign.is_none() || prev_sign == Some(sign)) {
                Some(Some(sign))
            } else {
                None
            }
        })
        .is_some()
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let levels = input.lines().map(|line| extract_numbers(line).collect_vec()).collect_vec();

    (levels.iter().filter(|l| safe(l.iter())).count(), levels.iter().filter(|l| safe2(l)).count())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
                "#,
            2,
            4
        );
    }
}
