use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    let chars = parse(input);

    println!("Part 1: {:?}", find_marker(&chars, 4));
    println!("Part 2: {:?}", find_marker(&chars, 14));
}

fn parse(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn find_marker(chars: &Vec<char>, window_size: usize) -> usize {
    chars
        .windows(window_size)
        .enumerate()
        .find(|(_i, window)| {
            let mut chars: HashSet<char> = HashSet::new();
            chars.extend(*window);
            chars.len() == window_size
        })
        .map(|(position, _)| position + window_size)
        .unwrap()
}
