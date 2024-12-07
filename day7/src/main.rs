use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::*;

fn check(target: usize, numbers: &[usize], allow_concat: bool) -> bool {
    check_impl(target, &numbers[1..], numbers[0], allow_concat)
}

fn check_impl(target: usize, numbers: &[usize], sum: usize, allow_concat: bool) -> bool {
    if sum > target {
        false
    } else if numbers.is_empty() {
        sum == target
    } else {
        check_impl(target, &numbers[1..], sum + numbers[0], allow_concat)
            || check_impl(target, &numbers[1..], sum * numbers[0], allow_concat)
            || allow_concat
                && check_impl(
                    target,
                    &numbers[1..],
                    (sum.to_string() + &numbers[0].to_string()).parse().unwrap(),
                    allow_concat,
                )
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let lines = input
        .lines()
        .map(|line| {
            let (target, numbers) = line.split_once(": ").unwrap();
            let numbers = extract_numbers_unsigned(numbers);
            (target.parse::<usize>().unwrap(), numbers.collect_vec())
        })
        .collect_vec();

    let part1 = lines
        .iter()
        .filter_map(|(target, numbers)| check(*target, numbers, false).then_some(*target))
        .sum_usize();

    let part2 = lines
        .iter()
        .filter_map(|(target, numbers)| check(*target, numbers, true).then_some(*target))
        .sum_usize();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(r#"190: 10 19"#, 190);

        assert_example!(
            r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
                "#,
            3749,
            11387
        );
    }
}
