fn main() {
    let input = include_str!("input.txt");
    let lines = input.lines().collect();

    println!("Part 1: {}", part_one(&lines));
    println!("Part 2: {}", part_two(&lines));
}

fn part_one(lines: &Vec<&str>) -> u32 {
    lines
        .iter()
        .map(|l| l.chars().flat_map(|c| c.to_digit(10)).collect::<Vec<u32>>())
        .map(|digits| digits[0] * 10 + digits[digits.len() - 1])
        .sum()
}

fn part_two(lines: &Vec<&str>) -> usize {
    let ds = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    lines
        .iter()
        .map(|l| {
            let mut digits: Vec<usize> = vec![];

            for (i, c) in l.chars().enumerate() {
                if let Some(d) = c.to_digit(10) {
                    digits.push(d.try_into().unwrap())
                } else {
                    for (d, s) in ds.iter().enumerate() {
                        if l[i..].starts_with(s) {
                            digits.push(d + 1)
                        }
                    }
                }
            }

            digits
        })
        .map(|digits| digits[0] * 10 + digits[digits.len() - 1])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("test.txt");
        let lines = input.lines().collect();
        assert_eq!(part_one(&lines), 142);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("test2.txt");
        let lines = input.lines().collect();
        assert_eq!(part_two(&lines), 281);
    }
}
