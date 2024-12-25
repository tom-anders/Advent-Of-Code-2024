use aoc_derive::aoc_main;
use grid::Grid;
use itertools::iproduct;
use utils::*;

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let (locks, keys): (Vec<_>, Vec<_>) = input
        .blocks()
        .map(|block| Grid::<char>::from_iter(block.lines().map(|l| l.chars())))
        .partition(|grid| grid.row(0).all(|(_, &c)| c == '#'));

    iproduct!(locks, keys)
        .filter(|(lock, key)| {
            lock.cols().zip(key.cols()).all(|(lhs, rhs)| {
                lhs.values().filter(|&&c| c == '#').count()
                    + rhs.values().filter(|&&c| c == '#').count()
                    <= 7
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#,
            3
        );
    }
}
