use std::{collections::HashSet, str::FromStr};

use aoc_derive::aoc_main;
use itertools::{Itertools, iproduct};
use utils::*;

#[derive(Debug, Clone)]
struct Vm {
    memory: Vec<usize>,
    ip: usize,
    ra: usize,
    rb: usize,
    rc: usize,
}

impl FromStr for Vm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();

        Ok(Self {
            memory: extract_numbers(lines[4]).collect(),
            ip: 0,
            ra: extract_numbers_unsigned(lines[0]).next().unwrap(),
            rb: extract_numbers_unsigned(lines[1]).next().unwrap(),
            rc: extract_numbers_unsigned(lines[2]).next().unwrap(),
        })
    }
}

impl Vm {
    fn combo(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.ra,
            5 => self.rb,
            6 => self.rc,
            _ => unreachable!(),
        }
    }

    fn clone_with_ra(&self, ra: usize) -> Self {
        let mut vm = self.clone();
        vm.ra = ra;
        vm
    }

    fn run(&mut self) -> Vec<usize> {
        let mut output = vec![];

        loop {
            let Some(opcode) = self.memory.get(self.ip) else { break };
            let operand = self.memory[self.ip + 1] as usize;
            match self.memory[self.ip] {
                0 => self.ra = self.ra / 2_usize.pow(self.combo(operand) as u32),
                1 => self.rb = self.rb ^ operand,
                2 => self.rb = self.combo(operand) % 8,
                3 => {
                    if self.ra != 0 {
                        self.ip = operand;
                        continue;
                    }
                }
                4 => self.rb = self.rb ^ self.rc,
                5 => output.push(self.combo(operand) % 8),
                6 => self.rb = self.ra / 2_usize.pow(self.combo(operand) as u32),
                7 => self.rc = self.ra / 2_usize.pow(self.combo(operand) as u32),
                _ => unreachable!("invalid opcode {opcode}"),
            }

            self.ip += 2;
        }

        output
    }
}

fn find_candidates(mut vm: Vm, output_index: usize) -> HashSet<usize> {
    let expected = vm.memory[output_index];

    // Remove the final loop
    *vm.memory.last_mut().unwrap() = vm.memory.len();

    iproduct!(0..=0b111, 0..=0b111)
        .filter_map(|(last_three_bits, middle_bits)| {
            let middle_shift = last_three_bits ^ 1;

            let ra = last_three_bits + (middle_bits << middle_shift);

            (vm.clone_with_ra(ra).run()[0] == expected).then_some(ra << (3 * output_index))
        })
        .collect()
}

fn part1(mut vm: Vm) -> String {
    vm.run().into_iter().join(",")
}

// Why this works (without posting the whole input, which would be against the rules):
// For my input (and probably also for the other ones), we can make a few key observations:
// - The program loops until register A contains 0
// - Register A is shifted 3 bits to the right at the end of every iteration
// - For each iteration, the output value depends only on (A % 8) and ((A % 8) ^ 1) << 3
//
// So, to check if a register A value produces the correct output, we just need to check the last 3
// bits and the three ones that result from the bitshift. This is what find_candidates() does.
// After the shift, the two bit-triples may overlap, but that's ok, either that means the solution
// does not (so we filter it out), or it does work by accident.
//
// Finally, for every index in the output we can then find the possible values for A that would
// produce the right output. We can then bitwise OR all combinations - If that still produces the
// correct output, we have a match! (and then need to find the minimum).
// To reduce the search space, instead of testing all possible combinations, we start by only
// checking two output indices, filtering out the candidates that don't produce the correct output.
// Then keep going until we have our final set of candidates
fn part2(vm: Vm) -> usize {
    (0..vm.memory.len())
        .fold(HashSet::from([0_usize]), |candidates, output_index| {
            let next = find_candidates(vm.clone(), output_index);

            iproduct!(candidates.into_iter(), next.iter())
                .filter_map(|(first, second)| {
                    let output = vm.clone_with_ra(first | second).run();
                    let upper_bound = (output_index + 1).min(output.len());
                    (output[..upper_bound] == vm.memory[..upper_bound]).then_some(first | second)
                })
                .collect()
        })
        .into_iter()
        .filter(|&candidate| vm.clone_with_ra(candidate).run() == vm.memory)
        .min()
        .unwrap()
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let vm: Vm = input.parse();

    (part1(vm.clone()), part2(vm))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        assert_eq!(
            part1(
                Input::from(
                    "Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0",
                )
                .parse()
            ),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }
}
