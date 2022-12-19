use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");
    let instructions = parse(input);
    println!("Part 1: {:?}", sum_of_signal_strenghts(instructions));
}

type Program = VecDeque<Op>;

#[derive(Debug)]
enum Op {
    Noop,
    Add(i32),
}

#[derive(Debug)]
struct VirtualMachine {
    cycle: i32,
    x: i32,
}

impl VirtualMachine {
    fn new() -> Self {
        Self { cycle: 0, x: 0 }
    }

    fn exec(&mut self, mut stack: Program, mut callback: impl FnMut(i32, i32)) {
        self.cycle = 1;
        self.x = 1;

        while !stack.is_empty() {
            let op = stack.pop_front().unwrap();

            match op {
                Op::Noop => {
                    self.cycle += 1;
                    callback(self.cycle, self.x);
                }
                Op::Add(x) => {
                    self.cycle += 1;
                    callback(self.cycle, self.x);

                    self.cycle += 1;
                    self.x += x;
                    callback(self.cycle, self.x);
                }
            };
        }
    }
}

fn parse(input: &str) -> Program {
    input
        .lines()
        .filter_map(|l| {
            if l == "noop" {
                Some(Op::Noop)
            } else {
                let (_, value) = l.split_once(' ')?;
                Some(Op::Add(value.parse().ok()?))
            }
        })
        .collect()
}

fn sum_of_signal_strenghts(program: Program) -> i32 {
    let mut signal_strengths = vec![];

    let mut vm = VirtualMachine::new();

    vm.exec(program, |cycle, x| {
        println!("cycle {} x {}", cycle, x);

        if cycle == 20 || cycle % 40 == 20 {
            signal_strengths.push(cycle * x);
        }
    });

    signal_strengths.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("test.txt");
        let instructions = parse(input);
        assert_eq!(sum_of_signal_strenghts(instructions), 13140);
    }
}
