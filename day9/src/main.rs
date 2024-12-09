#![feature(let_chains)]

use std::str::FromStr;

use aoc_derive::aoc_main;
use utils::*;

fn part1(mut disk: Vec<Option<usize>>) -> usize {
    loop {
        let first_none =
            disk.iter().enumerate().find_map(|(i, c)| c.is_none().then_some(i)).unwrap();

        if disk[first_none..].iter().all(|c| c.is_none()) {
            return disk.iter().enumerate().flat_map(|(i, c)| c.map(|c| i * c)).sum();
        }

        let back_index =
            disk.iter().enumerate().rev().find_map(|(i, c)| c.is_some().then_some(i)).unwrap();
        let insert_index =
            disk.iter().enumerate().find_map(|(i, c)| c.is_none().then_some(i)).unwrap();

        disk.swap(back_index, insert_index);
    }
}

#[derive(Debug, Clone)]
struct File {
    id: usize,
    size: usize,
    tried_to_move: bool,
}

#[derive(Debug, Clone, derive_more::Unwrap)]
#[unwrap(owned, ref, ref_mut)]
enum Block {
    Space(usize),
    File(File),
}

#[derive(Debug, Clone, derive_more::Deref, derive_more::DerefMut)]
struct Disk {
    blocks: Vec<Block>,
}

impl FromStr for Disk {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            blocks: s
                .trim()
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    let size = c.to_string().parse().unwrap();
                    if i % 2 == 0 {
                        Block::File(File { id: i / 2, size, tried_to_move: false })
                    } else {
                        Block::Space(size)
                    }
                })
                .collect(),
        })
    }
}

impl Disk {
    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| !matches!(block, Block::Space(0)))
            .fold((0, 0), |(pos, checksum), b| match b {
                Block::File(File { id, size, .. }) => {
                    (pos + size, checksum + id * (pos..pos + size).sum::<usize>())
                }
                Block::Space(size) => (pos + size, checksum),
            })
            .1
    }

    fn next_file_to_move(&mut self) -> Option<(usize, File)> {
        self.iter_mut().enumerate().rev().find_map(|(i, block)| {
            if let Block::File(file) = block
                && !file.tried_to_move
            {
                file.tried_to_move = true;
                Some((i, file.clone()))
            } else {
                None
            }
        })
    }

    fn find_insertion_index_part2(&self, file: &File, max_index: usize) -> Option<usize> {
        self[..max_index].iter().enumerate().find_map(|(i, b)| {
            if let Block::Space(space) = b
                && *space >= file.size
            {
                Some(i)
            } else {
                None
            }
        })
    }
}

fn part2(mut disk: Disk) -> usize {
    loop {
        if let Some((next_file_index, file)) = disk.next_file_to_move() {
            if let Some(insertion_index) = disk.find_insertion_index_part2(&file, next_file_index) {
                *disk[insertion_index].unwrap_space_mut() -= file.size;
                disk[next_file_index] = Block::Space(file.size);
                disk.insert(insertion_index, Block::File(file));
            }
        } else {
            return disk.checksum();
        }
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    (
        part1(
            input
                .raw
                .trim()
                .chars()
                .enumerate()
                .flat_map(|(i, c)| {
                    (0..(c.to_string().parse().unwrap()))
                        .map(move |_| if i % 2 == 0 { Some(i / 2) } else { None })
                })
                .collect(),
        ),
        part2(input.parse()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(r#"2333133121414131402"#, 1928, 2858);
    }
}
