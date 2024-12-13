use std::str::FromStr;

use aoc_derive::aoc_main;
use itertools::Itertools;
use math::Vec2D;
use utils::*;

#[derive(Debug, Clone)]
struct Machine {
    a: Vec2D,
    b: Vec2D,
    prize: Vec2D,
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b, prize) = s.lines().map(extract_two_numbers).collect_tuple().unwrap();
        Ok(Self { a, b, prize })
    }
}

impl Machine {
    fn into_part2(mut self) -> Self {
        self.prize += Vec2D::new(10000000000000_i64, 10000000000000_i64);
        self
    }

    fn solve(&self) -> Option<i64> {
        let denominator = self.a.x * self.prize.y - self.prize.x * self.a.y;
        let numerator = self.a.x * self.b.y - self.b.x * self.a.y;

        (denominator % numerator == 0).then_some(denominator / numerator).and_then(|num_b| {
            let denominator = self.prize.x - num_b * self.b.x;
            (denominator % self.a.x == 0).then_some(num_b + 3 * denominator / self.a.x)
        })
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let machines = input.blocks().flat_map(Machine::from_str).collect_vec();

    (
        machines.iter().flat_map(Machine::solve).sum_usize(),
        machines.into_iter().flat_map(|m| m.into_part2().solve()).sum_usize(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
                        "#,
            480
        );
    }
}
