use std::cmp;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn parse(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .lines()
        .map(|line| {
            let o: Vec<(usize, usize, usize)> = line
                .split(": ")
                .nth(1)
                .expect("a line has two parts")
                .split("; ")
                .map(|split| {
                    let (mut red, mut green, mut blue) = (0, 0, 0);

                    split.split(",").for_each(|s| {
                        let s = s.trim();
                        let (count, color) = s.split_at(s.find(" ").unwrap());
                        let count = count.trim().parse::<usize>().unwrap();
                        let color = color.trim();

                        match color {
                            "red" => red += count,
                            "green" => green += count,
                            "blue" => blue += count,
                            _ => panic!("unknown color"),
                        }
                    });

                    (red, green, blue)
                })
                .collect();

            o.iter().fold((0, 0, 0), |a, b| {
                (cmp::max(a.0, b.0), cmp::max(a.1, b.1), cmp::max(a.2, b.2))
            })
        })
        .collect()
}

fn part_one(input: &str) -> usize {
    parse(input)
        .iter()
        .enumerate()
        .filter(|(_, (red, green, blue))| red <= &12 && green <= &13 && blue <= &14)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part_two(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|(red, green, blue)| red * green * blue)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("test.txt");
        assert_eq!(part_one(input), 8);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("test.txt");
        assert_eq!(part_two(input), 2286);
    }
}
