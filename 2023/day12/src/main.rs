fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_one(input));
}

fn parse(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let springs = parts.nth(0).unwrap().chars().collect::<Vec<_>>();

            let groups = parts
                .nth(0)
                .unwrap()
                .split(",")
                .map(|s| s.parse::<usize>().expect("valid integer"))
                .collect::<Vec<_>>();

            (springs, groups)
        })
        .collect()
}

fn arrangements(springs: &[char], groups: &[usize]) -> usize {
    // when we've consumed all springs
    if springs.is_empty() {
        // we have a valid arrangement if:
        // there are no groups left
        return if groups.is_empty() { 1 } else { 0 };
    }

    // when we've consumed all groups
    if groups.is_empty() {
        // we have a valid arrangement if:
        // the remaining springs must all be operational
        return if !springs.contains(&'#') { 1 } else { 0 };
    }

    // if there are not enough springs to cover the blocks,
    // then there are no arrangements.
    if springs.len() < groups.iter().sum::<usize>() + groups.len() - 1 {
        return 0;
    }

    // keep track of the number of valid arrangements
    let mut count = 0;

    // treat '?' as '.'
    if springs[0] == '.' || springs[0] == '?' {
        count += arrangements(&springs[1..], groups);
    }

    // treat '?' as '#'
    if springs[0] == '#' || springs[0] == '?' {
        // check if we have enough springs left to satisfy the current group
        // and check if there are no operational springs (`.`) in the next `groups[0]` springs (chars)
        // and check if the spring after the damaged springs group is operational (`.`) or if there
        // are no springs left.
        if groups[0] <= springs.len()
            && !springs[..groups[0]].contains(&'.')
            && (springs.len() == groups[0] || springs[groups[0]] != '#')
        {
            let n = springs.len().min(groups[0] + 1);
            count += arrangements(&springs[n..], &groups[1..]);
        }
    }

    count
}

fn part_one(input: &str) -> usize {
    let records = parse(input);

    records
        .iter()
        .map(|(springs, groups)| arrangements(springs, groups))
        .sum()
}

#[test]
fn test_part_one() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    assert_eq!(part_one(input), 21);
}

#[test]
fn test_arrangement_a() {
    assert_eq!(
        arrangements(&vec!['?', '?', '?', '.', '#', '#', '#'], &vec![1, 1, 3]),
        1
    );
}

#[test]
fn test_arrangement_b() {
    assert_eq!(
        arrangements(
            &vec!['.', '?', '?', '.', '.', '?', '?', '.', '.', '.', '?', '#', '#', '.'],
            &vec![1, 1, 3]
        ),
        4
    );
}

#[test]
fn test_arrangement_c() {
    assert_eq!(
        arrangements(
            &vec!['?', '#', '?', '#', '?', '#', '?', '#', '?', '#', '?', '#', '?', '#', '?'],
            &vec![1, 3, 1, 6]
        ),
        1
    );
}

#[test]
fn test_arrangement_d() {
    assert_eq!(
        arrangements(
            &vec!['?', '?', '?', '?', '.', '#', '.', '.', '.', '#', '.', '.', '.'],
            &vec![4, 1, 1]
        ),
        1
    );
}

#[test]
fn test_arrangement_e() {
    assert_eq!(
        arrangements(
            &vec![
                '?', '?', '?', '?', '.', '#', '#', '#', '#', '#', '#', '.', '.', '#', '#', '#',
                '#', '#', '.'
            ],
            &vec![1, 6, 5]
        ),
        4
    );
}

#[test]
fn test_arrangement_f() {
    assert_eq!(
        arrangements(
            &vec!['?', '#', '#', '#', '?', '?', '?', '?', '?', '?', '?', '?'],
            &vec![3, 2, 1]
        ),
        10
    );
}
