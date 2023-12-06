use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[derive(PartialEq)]
struct MapEntry {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl FromStr for MapEntry {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        let destination_range_start = parts[0].parse()?;
        let source_range_start = parts[1].parse()?;
        let range_length = parts[2].parse()?;

        Ok(MapEntry {
            destination_range_start,
            source_range_start,
            range_length,
        })
    }
}

impl fmt::Debug for MapEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "dst: {}, src: {}, len: {}",
            self.destination_range_start, self.source_range_start, self.range_length
        )
    }
}

fn parse(input: &str) -> (Vec<usize>, Vec<(&str, Vec<MapEntry>)>) {
    let mut parts: Vec<_> = input.split("\n\n").collect();

    let seeds = parts
        .remove(0)
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let maps = parts
        .iter()
        .map(|part| {
            let mut parts = part.split("\n").collect::<Vec<_>>();

            let id = parts.remove(0).split(" map:").nth(0).unwrap();

            let map_entries = part
                .split("\n")
                .skip(1)
                .flat_map(|l| l.parse::<MapEntry>())
                .collect::<Vec<_>>();

            (id, map_entries)
        })
        .collect::<Vec<_>>();

    (seeds, maps)
}

fn part_one(input: &str) -> usize {
    let (seeds, maps) = parse(input);

    let locations = seeds
        .iter()
        .map(|s| {
            println!("\nSeed {}", s);

            maps.iter().fold(*s, |acc, (map_id, map_entries)| {
                println!("{}", map_id);

                let mut distination_number = acc;

                for map_entry in map_entries {
                    if acc >= map_entry.source_range_start
                        && acc <= map_entry.source_range_start + map_entry.range_length
                    {
                        println!(
                            "{} is in range {}-{}",
                            acc,
                            map_entry.source_range_start,
                            map_entry.source_range_start + map_entry.range_length
                        );

                        distination_number =
                            acc + map_entry.destination_range_start - map_entry.source_range_start;

                        break;
                    }
                }

                println!("Distination number {}", distination_number);

                distination_number
            })
        })
        .collect::<Vec<usize>>();

    // find the lowest location number that corresponds to any of the initial seeds.
    locations.iter().min().unwrap().clone()
}

#[test]
fn test_part_one() {
    let input = include_str!("test.txt");
    assert_eq!(part_one(input), 35);
}

fn part_two(input: &str) -> usize {
    let (seeds, maps) = parse(input);

    let seed_ranges = seeds
        .chunks(2)
        .flat_map(|chunk| {
            let start = chunk[0];
            let end = start + chunk[1];

            start..end
        })
        .collect::<Vec<_>>();

    seed_ranges
        .par_iter()
        .map(|s| find_location(&maps, *s))
        .min()
        .unwrap()
}

fn find_location(maps: &Vec<(&str, Vec<MapEntry>)>, location: usize) -> usize {
    maps.iter()
        .fold(location, |loc, (_, map)| transform(loc, map))
}

fn transform(location: usize, map: &[MapEntry]) -> usize {
    map.iter()
        .find_map(|e| convert(location, e))
        .unwrap_or(location)
}

fn convert(location: usize, e: &MapEntry) -> Option<usize> {
    let lower_bound = e.source_range_start;
    let upper_bound = e.source_range_start + e.range_length;
    let bounds = lower_bound..=upper_bound;

    if !bounds.contains(&location) {
        return None;
    }

    Some(e.destination_range_start + location - e.source_range_start)
}

#[test]
fn test_part_two() {
    let input = include_str!("test.txt");
    assert_eq!(part_two(input), 46);
}
