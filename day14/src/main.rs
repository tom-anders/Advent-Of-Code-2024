use std::str::FromStr;

use aoc_derive::aoc_main;
use itertools::Itertools;
use lazy_regex::regex_is_match;
use math::Vec2D;
use utils::*;

#[derive(Debug, Clone)]
struct Robot {
    pos: Vec2D,
    velocity: Vec2D,
}

impl Robot {
    fn move_(mut self, width: i64, height: i64) -> Self {
        self.pos += self.velocity + (width, height);

        self.pos.x %= width;
        self.pos.y %= height;

        self
    }
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, vx, vy) = extract_numbers::<i64>(s).collect_tuple().unwrap();
        Ok(Self { pos: (x, y).into(), velocity: (vx, vy).into() })
    }
}

fn count_robots(robots: &[Robot], width: i64, height: i64) -> usize {
    robots.iter().filter(|r| r.pos.x < width / 2 && r.pos.y < height / 2).count()
        * robots.iter().filter(|r| r.pos.x < width / 2 && r.pos.y > height / 2).count()
        * robots.iter().filter(|r| r.pos.x > width / 2 && r.pos.y < height / 2).count()
        * robots.iter().filter(|r| r.pos.x > width / 2 && r.pos.y > height / 2).count()
}

fn part1(robots: impl Iterator<Item = Robot>, width: i64, height: i64) -> usize {
    let robots = robots
        .into_iter()
        .map(|robot| (0..100).fold(robot, |robot, _| robot.move_(width, height)))
        .collect_vec();
    count_robots(&robots, width, height)
}

fn part2(mut robots: Vec<Robot>, width: usize, height: usize) -> usize {
    (1..)
        .find(|_| {
            robots =
                robots.clone().into_iter().map(|r| r.move_(width as i64, height as i64)).collect();

            let lines = (0..height)
                .map(|y| {
                    (0..width)
                        .map(|x| if robots.iter().any(|r| r.pos == (x, y)) { 'X' } else { ' ' })
                        .collect::<String>()
                })
                .collect_vec();

            if lines.iter().any(|line| regex_is_match!(r"X{10,}", line)) {
                for line in lines {
                    println!("{line}");
                }
                println!();
                true
            } else {
                false
            }
        })
        .unwrap()
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let robots = input.parse_lines::<Robot>();

    (part1(robots.clone(), 101, 103), part2(robots.clone().collect(), 101, 103))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        let example = "
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
            ";
        assert_eq!(part1(Input::from(example).parse_lines(), 11, 7), 12);
    }
}
