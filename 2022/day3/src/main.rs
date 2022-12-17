fn main() {
    let input = include_str!("./input.txt");

    let rucksacks: Vec<&str> = input.lines().collect();

    println!("Part 1: {:?}", sum_of_priorities(&rucksacks));
    println!("Part 2: {:?}", sum_of_priorities_2(&rucksacks));
}

fn sum_of_priorities(rucksacks: &Vec<&str>) -> u32 {
    rucksacks
        .iter()
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .filter_map(|rucksack| {
            let compartment_a = rucksack.0.as_bytes();
            let compartment_b = rucksack.1.as_bytes();

            compartment_a
                .iter()
                .find(|byte| compartment_b.contains(byte))
                .map(|&byte| item_priority(byte) as u32)
        })
        .sum()
}

fn sum_of_priorities_2(rucksacks: &Vec<&str>) -> u32 {
    rucksacks
        .chunks(3)
        .filter_map(|group| {
            let mut group = group.iter();
            let a = group.next()?.as_bytes();
            let b = group.next()?.as_bytes();
            let c = group.next()?.as_bytes();

            a.iter()
                .find(|byte| b.contains(byte) && c.contains(byte))
                .map(|&byte| item_priority(byte) as u32)
        })
        .sum()
}

fn item_priority(code: u8) -> u8 {
    code % 32 + (26 * (code <= 90) as u8)
}
