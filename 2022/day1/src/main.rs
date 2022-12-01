fn main() {
    let input = include_str!("./input.txt");
    let lines = input.split("\n\n");

    let mut calories_per_elf: Vec<u32> = lines
        .map(|line| line.split("\n").flat_map(|str| str.parse::<u32>()).sum())
        .collect();

    calories_per_elf.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {:?}", calories_per_elf[0]);
    println!(
        "Part 2: {:?}",
        calories_per_elf[0] + calories_per_elf[1] + calories_per_elf[2]
    );
}
