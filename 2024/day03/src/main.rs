use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|cap| cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap())
        .sum()
}

fn part_two(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();

    re.captures_iter(input)
        .fold((0, true), |(acc, enabled), cap| {
            if cap.get(3).is_some() {
                (acc, true)
            } else if cap.get(4).is_some() {
                (acc, false)
            } else {
                (
                    acc + if enabled {
                        cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap()
                    } else {
                        0
                    },
                    enabled,
                )
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part_one(input), 161);
    }

    #[test]
    fn test_part_two() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part_two(input), 48);
    }
}
