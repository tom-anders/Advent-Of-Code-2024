use aoc_derive::aoc_main;
use grid::Grid;
use itertools::Itertools;
use math::Vec2D;
use utils::*;

fn try_move_part1(pos: Vec2D, heading: Vec2D, map: &mut Grid<char>) -> bool {
    match map[pos] {
        '.' => {
            map.swap(pos, pos - heading);
            true
        }
        '#' => false,
        'O' => {
            let can_move = try_move_part1(pos + heading, heading, map);
            if can_move {
                map.swap(pos, pos - heading);
            }
            can_move
        }
        _ => unreachable!(),
    }
}

fn part1(mut map: Grid<char>, instructions: &str) -> i64 {
    let mut pos = map.iter().find_map(|(pos, &c)| (c == '@').then_some(pos)).unwrap();
    map[pos] = '.';

    for heading in instructions.chars().filter(|c| !c.is_ascii_whitespace()).map(|instr| {
        Vec2D::from(match instr {
            'v' => (0, 1),
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            _ => unreachable!(),
        })
    }) {
        if try_move_part1(pos + heading, heading, &mut map) {
            pos += heading;
        }
    }

    map.iter().filter_map(|(pos, &c)| (c == 'O').then_some(pos.x + 100 * pos.y)).sum()
}

fn try_move_horizontally_part2(pos: Vec2D, heading: Vec2D, map: &mut Grid<char>) -> bool {
    if let Some(dot) = (1..)
        .find_map(|i| match map[pos + i * heading] {
            '.' => Some(Some(i)),
            '#' => Some(None),
            _ => None,
        })
        .unwrap()
    {
        for j in (0..dot).rev() {
            map.swap(pos + (j + 1) * heading, pos + j * heading);
        }
        return true;
    }
    false
}

fn try_move_vertically_part2(pos: Vec2D, heading: Vec2D, map: &mut Grid<char>) -> bool {
    match map[pos + heading] {
        '#' => return false,
        '.' => {
            return true;
        }
        _ => (),
    }

    let mut to_move = vec![vec![
        pos + heading,
        pos + heading + if map[pos + heading] == ']' { (-1, 0) } else { (1, 0) },
    ]];
    loop {
        let upper_row = to_move.last().unwrap();

        if upper_row.iter().any(|&pos| map[pos + heading] == '#') {
            // Can't move
            return false;
        }

        if upper_row.iter().all(|&pos| map[pos + heading] == '.') {
            for row in to_move.into_iter().rev() {
                for pos in row {
                    map.swap(pos + heading, pos);
                }
            }

            return true;
        }

        to_move.push(
            upper_row
                .iter()
                .flat_map(|&pos| match map[pos + heading] {
                    ']' => vec![pos + heading, pos + heading + (-1, 0)],
                    '[' => vec![pos + heading, pos + heading + (1, 0)],
                    _ => vec![],
                })
                .unique()
                .collect(),
        );
    }
}

fn part2(map: &str, instructions: &str) -> i64 {
    let mut map = map
        .trim()
        .lines()
        .map(|line| {
            line.chars().flat_map(|c| {
                match c {
                    '#' => "##",
                    'O' => "[]",
                    '.' => "..",
                    '@' => "@.",
                    _ => unreachable!(),
                }
                .chars()
            })
        })
        .collect::<Grid<char>>();

    let mut pos = map.iter().find_map(|(pos, &c)| (c == '@').then_some(pos)).unwrap();
    map[pos] = '.';

    for heading in instructions.chars().filter(|c| !c.is_ascii_whitespace()).map(|instr| {
        Vec2D::from(match instr {
            'v' => (0, 1),
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            _ => unreachable!(),
        })
    }) {
        if heading.y == 0 && try_move_horizontally_part2(pos, heading, &mut map)
            || try_move_vertically_part2(pos, heading, &mut map)
        {
            pos += heading
        }
    }

    map.iter().filter_map(|(pos, &c)| (c == '[').then_some(pos.x + 100 * pos.y)).sum()
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let (map, instructions) = input.blocks().collect_tuple().unwrap();

    (part1(map.lines().map(str::chars).collect(), instructions), part2(map, instructions))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        part2(
            "
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######",
            "<vv<<^^<<^^",
        );

        assert_example!(
            "
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
            ",
            2028
        );

        assert_example!(
            r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
            10092,
            9021
        );
    }
}
