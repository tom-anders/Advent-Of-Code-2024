use aoc_derive::aoc_main;
use itertools::Itertools;
use lazy_regex::regex;
use utils::*;

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let instrs = regex!(r"(do\(\)|don't\(\)|mul\(\d\d?\d?,\d\d?\d?\))")
        .find_iter(&input.raw)
        .map(|m| m.as_str())
        .collect_vec();
    dbg!(&instrs);
    let mut sum = 0;
    let mut sum2 = 0;
    let mut doo = true;
    for i in instrs {
        if i == "do()" {
            doo = true;
        } else if i == "don't()" {
            doo = false;
        } else {
            let (a, b) = extract_numbers_unsigned(i).collect_tuple().unwrap();
            sum += a * b;
            if doo {
                sum2 += a * b;
            }
        }
    }
    (sum, sum2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
                "#,
            161
        );
        assert_part2!(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
            48
        );
    }
}
