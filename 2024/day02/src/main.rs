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

fn is_safe(report: &[usize]) -> bool {
    let mut safe = true;

    let delta = report[0] as i32 - report[1] as i32;
    let incr_or_decr: i32 = if delta.is_positive() { -1 } else { 1 };

    for pair in report.windows(2) {
        let a = pair[0] as i32;
        let b = pair[1] as i32;

        let delta = a - b;
        let abs_delta = delta.abs();

        if delta.is_positive() == incr_or_decr.is_positive() {
            safe = false;
            break;
        }

        if !(abs_delta >= 1 && abs_delta <= 3) {
            safe = false;
            break;
        }
    }

    safe
}

fn part_one(input: &str) -> usize {
    let reports = parse(input);

    reports.iter().filter(|report| is_safe(report)).count()
}

fn is_safe_with_dampener(report: &[usize]) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut report = report.to_vec();
        report.remove(i);

        if is_safe(&report) {
            return true;
        }
    }

    false
}

fn part_two(input: &str) -> usize {
    let reports = parse(input);

    reports
        .iter()
        .filter(|report| is_safe_with_dampener(report))
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
