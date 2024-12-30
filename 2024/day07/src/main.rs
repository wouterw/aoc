use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_once(": ").unwrap();

            let result = parts.0.parse().unwrap();

            let numbers = parts
                .1
                .split(" ")
                .map(|rule| rule.parse().unwrap())
                .collect();

            (result, numbers)
        })
        .collect()
}

#[derive(Clone, Debug)]
enum Op {
    Add,
    Multiply,
    Concat,
}

fn solve_equation(result: u64, numbers: Vec<u64>, operators: &Vec<Op>) -> Option<u64> {
    let k = numbers.len() - 1;
    let permutations = itertools::repeat_n(operators, k).multi_cartesian_product();

    for perm in permutations {
        let mut numbers = numbers.clone();
        let mut operators = perm.clone();

        while numbers.len() > 1 {
            let op = operators.pop().unwrap();

            let a = numbers.remove(0);
            let b = numbers.remove(0);

            let result = match op {
                Op::Add => a + b,
                Op::Multiply => a * b,
                Op::Concat => format!("{}{}", a, b).parse().unwrap(),
            };

            numbers.insert(0, result);
        }

        if numbers[0] == result {
            return Some(result);
        }
    }

    None
}

fn part_one(input: &str) -> u64 {
    let equations = parse(input);
    let operators = vec![Op::Add, Op::Multiply];

    equations
        .iter()
        .filter_map(|(result, numbers)| solve_equation(*result, numbers.clone(), &operators))
        .sum()
}

fn part_two(input: &str) -> u64 {
    let equations = parse(input);
    let operators = vec![Op::Add, Op::Multiply, Op::Concat];

    equations
        .iter()
        .filter_map(|(result, numbers)| solve_equation(*result, numbers.clone(), &operators))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

        assert_eq!(part_one(input), 3749);
    }

    #[test]
    fn test_part_two() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

        assert_eq!(part_two(input), 11387);
    }
}
