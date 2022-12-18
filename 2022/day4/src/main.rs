use std::ops::Range;

fn main() {
    let input = include_str!("./input.txt");

    let assignments: Vec<(Range<u8>, Range<u8>)> = input.lines().filter_map(parse).collect();

    println!("Part 1: {:?}", number_of_contained_pairs(&assignments));
    println!("Part 2: {:?}", number_of_overlapping_pairs(&assignments));
}

fn parse(assignment: &str) -> Option<(Range<u8>, Range<u8>)> {
    let (a, b) = assignment.split_once(',')?;
    let (a_start, a_end) = a.split_once('-')?;
    let (b_start, b_end) = b.split_once('-')?;

    Some((
        (a_start.parse().ok()?)..(a_end.parse().ok()?),
        (b_start.parse().ok()?)..(b_end.parse().ok()?),
    ))
}

fn subset(a: &Range<u8>, b: &Range<u8>) -> bool {
    b.start >= a.start && b.end <= a.end
}

fn overlap(a: &Range<u8>, b: &Range<u8>) -> bool {
    (b.start >= a.start && b.start <= a.end) || (a.start >= b.start && a.start <= b.end)
}

fn number_of_contained_pairs(assignments: &Vec<(Range<u8>, Range<u8>)>) -> usize {
    assignments
        .iter()
        .filter(|(a, b)| subset(a, b) || subset(b, a))
        .count()
}

fn number_of_overlapping_pairs(assignments: &Vec<(Range<u8>, Range<u8>)>) -> usize {
    assignments.iter().filter(|(a, b)| overlap(a, b)).count()
}
