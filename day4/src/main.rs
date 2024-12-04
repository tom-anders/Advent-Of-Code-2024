use aoc_derive::aoc_main;
use utils::*;

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let grid = input.char_grid();

    let mut part1 = 0;
    for x in 0..grid.num_cols() {
        for y in 0..grid.num_rows() {
            for inc_i in [-1_i64, 0, 1_i64] {
                for inc_j in [-1_i64, 0, 1_i64] {
                    if grid.get((x, y)) == Some(&'X')
                        && grid.get((x as i64 + inc_i, y as i64 + inc_j)) == Some(&'M')
                        && grid.get((x as i64 + 2 * inc_i, y as i64 + 2 * inc_j)) == Some(&'A')
                        && grid.get((x as i64 + 3 * inc_i, y as i64 + 3 * inc_j)) == Some(&'S')
                    {
                        part1 += 1;
                    }
                }
            }
        }
    }

    let mut part2 = 0;
    for x in 0..grid.num_cols() {
        for y in 0..grid.num_rows() {
            if grid.get((x + 1, y + 1)) == Some(&'A')
                && (grid.get((x, y)) == Some(&'M') && grid.get((x + 2, y + 2)) == Some(&'S')
                    || grid.get((x, y)) == Some(&'S') && grid.get((x + 2, y + 2)) == Some(&'M'))
                && (grid.get((x + 2, y)) == Some(&'M') && grid.get((x, y + 2)) == Some(&'S')
                    || grid.get((x + 2, y)) == Some(&'S') && grid.get((x, y + 2)) == Some(&'M'))
            {
                part2 += 1;
            }
        }
    }

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;

        assert_example!(
            r#"..X...
.SAMX.
.A..A.
XMAS.S
.X...."#,
            3 + 1
        );

        assert_example!(
            r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
                "#,
            "18"
        );
    }
}
