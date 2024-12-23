use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let parts = input.split_once("\n\n").unwrap();

    let rules = parts
        .0
        .lines()
        .map(|line| {
            let numbers = line.split_once("|").unwrap();
            (numbers.0.parse().unwrap(), numbers.1.parse().unwrap())
        })
        .collect();

    let series = parts
        .1
        .lines()
        .map(|line| line.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, series)
}

fn page_ordering_rules(rules: Vec<(u32, u32)>) -> HashMap<u32, Vec<u32>> {
    let mut ordering: HashMap<u32, Vec<u32>> = HashMap::new();

    for (a, b) in rules {
        ordering.entry(a).or_insert_with(Vec::new).push(b);
    }

    ordering
}

fn middle_page(pages: &Vec<u32>) -> u32 {
    let middle = (pages.len() as f32 / 2.0).ceil() as usize - 1;
    pages[middle]
}

fn is_correctly_ordered(rules: &HashMap<u32, Vec<u32>>, pages: &Vec<u32>) -> bool {
    for i in 0..pages.len() - 1 {
        let a = pages[i];
        let b = pages[i + 1];

        if !rules.get(&a).unwrap_or(&Vec::<u32>::new()).contains(&b) {
            return false;
        }
    }

    true
}

fn part_one(input: &str) -> u32 {
    let (rules, series) = parse_input(input);

    let ordering = page_ordering_rules(rules);

    let correctly_ordered_pages = series
        .iter()
        .filter(|pages| is_correctly_ordered(&ordering, pages));

    correctly_ordered_pages
        .map(|pages| middle_page(pages))
        .sum()
}

fn part_two(input: &str) -> u32 {
    let (rules, series) = parse_input(input);

    let ordering = page_ordering_rules(rules);

    let incorrectly_ordered_pages = series
        .iter()
        .filter(|pages| !is_correctly_ordered(&ordering, pages));

    let corrected_page_updates = incorrectly_ordered_pages.map(|pages| {
        let mut updated_pages = pages.clone();

        let mut i: usize = 0;

        let default = &Vec::<u32>::new();

        while i < pages.len() - 1 {
            let a = updated_pages[i];
            let b = updated_pages[i + 1];

            let rules = ordering.get(&a).unwrap_or(default);

            if !rules.contains(&b) {
                updated_pages[i] = b;
                updated_pages[i + 1] = a;
                i = 0;
            } else {
                i += 1;
            }
        }

        updated_pages
    });

    corrected_page_updates
        .map(|pages| middle_page(&pages))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

        assert_eq!(part_one(input), 143);
    }

    #[test]
    fn test_part_two() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

        assert_eq!(part_two(input), 123);
    }
}
