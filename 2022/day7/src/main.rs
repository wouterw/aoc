fn main() {
    let input = include_str!("input.txt");
    let lines = parse(input);

    println!("Part 1: {:?}", sum_of_directories(&lines));
    println!("Part 2: {:?}", directory_to_delete(&lines));
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn sum_of_directories(lines: &Vec<&str>) -> usize {
    let mut stack = vec![];
    let mut total = 0;

    for l in lines {
        let parts = l.split(" ").collect::<Vec<_>>();

        match (parts[0], parts[1]) {
            ("$", "ls") => {}
            ("$", "cd") => match parts[2] {
                "/" => stack.push(("/", 0)),
                ".." => {
                    let (_name, size) = stack.pop().unwrap();
                    stack.last_mut().unwrap().1 += size;

                    if size <= 100_000 {
                        total += size;
                    }
                }
                name => stack.push((name, 0)),
            },
            ("dir", _name) => {}
            (size, _name) => {
                let size = size.parse::<usize>().unwrap();
                stack.last_mut().unwrap().1 += size;
            }
        }

        println!("cmd {:?}", parts);
        println!("stack {:?}", stack);
    }

    while stack.len() > 1 {
        let (_name, size) = stack.pop().unwrap();
        stack.last_mut().unwrap().1 += size;
        println!("stack {:?}", stack);
    }

    total
}

fn directory_to_delete(lines: &Vec<&str>) -> usize {
    let mut stack = vec![];
    let mut directories = vec![];

    for l in lines {
        let parts = l.split(" ").collect::<Vec<_>>();

        match (parts[0], parts[1]) {
            ("$", "ls") => {}
            ("$", "cd") => match parts[2] {
                "/" => stack.push(("/", 0)),
                ".." => {
                    let (name, size) = stack.pop().unwrap();
                    stack.last_mut().unwrap().1 += size;
                    directories.push((name, size));
                }
                name => stack.push((name, 0)),
            },
            ("dir", _name) => {}
            (size, _name) => {
                let size = size.parse::<usize>().unwrap();
                stack.last_mut().unwrap().1 += size;
            }
        }

        println!("cmd {:?}", parts);
        println!("stack {:?}", stack);
    }

    while stack.len() > 1 {
        let (name, size) = stack.pop().unwrap();
        stack.last_mut().unwrap().1 += size;
        directories.push((name, size));
        println!("stack {:?}", stack);
    }

    let total_disk_space: usize = 70_000_000;
    let required_disk_space: usize = 30_000_000;
    let used_disk_space = stack.first().unwrap().1;
    let free_disk_space = total_disk_space - used_disk_space;
    let delta = required_disk_space - free_disk_space;

    println!("{:?}", directories);

    directories
        .into_iter()
        .filter(move |(_, size)| *size >= delta)
        .map(|(_, size)| size)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("test.txt");
        let lines = parse(input);
        assert_eq!(sum_of_directories(&lines), 95437);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("test.txt");
        let lines = parse(input);
        assert_eq!(directory_to_delete(&lines), 24933642);
    }
}
