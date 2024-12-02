fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn part_one(input: &str) -> usize {
    let reports = parse(input);

    reports
        .iter()
        .filter(|report| {
            let mut safe = true;
            let mut incr_or_decr: i32 = 0;

            for pair in report.windows(2) {
                let a = pair[0] as i32;
                let b = pair[1] as i32;

                let delta = a - b;
                let abs_delta = delta.abs();

                if incr_or_decr != 0 && delta.is_positive() == incr_or_decr.is_positive() {
                    safe = false;
                    break;
                }

                incr_or_decr = if delta.is_positive() { -1 } else { 1 };

                if !(abs_delta >= 1 && abs_delta <= 3) {
                    safe = false;
                    break;
                }
            }

            safe
        })
        .count()
}

fn part_two(input: &str) -> usize {
    let reports = parse(input);

    reports
        .iter()
        .filter(|report| {
            let mut number_of_bad_levels = 0;
            let mut incr_or_decr: i32 = 0;

            for pair in report.windows(2) {
                let a = pair[0] as i32;
                let b = pair[1] as i32;

                let delta = a - b;
                let abs_delta = delta.abs();

                if incr_or_decr != 0 && delta.is_positive() == incr_or_decr.is_positive() {
                    number_of_bad_levels += 1;
                }

                incr_or_decr = if delta.is_positive() { -1 } else { 1 };

                if !(abs_delta >= 1 && abs_delta <= 3) {
                    number_of_bad_levels += 1;
                }
            }

            number_of_bad_levels < 2
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        assert_eq!(part_one(input), 2);
    }

    #[test]
    fn test_part_two() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        assert_eq!(part_two(input), 4);
    }
}
