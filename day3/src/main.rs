use aoc_derive::aoc_main;
use lazy_regex::regex;
use utils::*;

#[derive(Debug, Copy, Clone)]
enum Instr {
    Mul(usize, usize),
    Do,
    Dont,
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let instrs = (0..input.len()).filter_map(|i| {
        let s = &input.as_str()[i..];
        if s.starts_with("do()") {
            Some(Instr::Do)
        } else if s.starts_with("don't()") {
            Some(Instr::Dont)
        } else {
            regex!(r"^mul\((\d{1,3}),(\d{1,3})\)").captures(s).map(|captures| {
                Instr::Mul(captures[1].parse().unwrap(), captures[2].parse().unwrap())
            })
        }
    });

    let part1 = instrs
        .clone()
        .filter_map(|instr| if let Instr::Mul(lhs, rhs) = instr { Some(lhs * rhs) } else { None })
        .sum_usize();

    let part2 = instrs
        .fold((0, true), |(sum, doit), instr| match instr {
            Instr::Mul(lhs, rhs) => (sum + if doit { lhs * rhs } else { 0 }, doit),
            Instr::Do => (sum, true),
            Instr::Dont => (sum, false),
        })
        .0;

    (part1, part2)
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
