fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut dims = line
                .split('x')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            dims.sort();

            let (l, w, h) = (dims[0], dims[1], dims[2]);

            let surface_area = 2 * l * w + 2 * w * h + 2 * h * l;
            let extra = l * w;

            surface_area + extra
        })
        .sum()
}

#[test]
fn test_part_one() {
    assert_eq!(part_one("2x3x4"), 58);
    assert_eq!(part_one("1x1x10"), 43);
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut dims = line
                .split('x')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            dims.sort();

            let (l, w, h) = (dims[0], dims[1], dims[2]);

            let ribbon_for_present = l + l + w + w;
            let ribbon_for_bow = l * w * h;

            ribbon_for_present + ribbon_for_bow
        })
        .sum()
}

#[test]
fn test_part_two() {
    assert_eq!(part_two("2x3x4"), 34);
    assert_eq!(part_two("1x1x10"), 14);
}
