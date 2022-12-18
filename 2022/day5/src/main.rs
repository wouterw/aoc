fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {:?}", part_1(input));
    println!("Part 2: {:?}", part_2(input));
}

fn part_1(input: &str) -> String {
    let (mut stack, moves) = parse(input);
    move_stacks(&mut stack, &moves, 9000);
    crates_at_top(&stack)
}

fn part_2(input: &str) -> String {
    let (mut stack, moves) = parse(input);
    move_stacks(&mut stack, &moves, 9001);
    crates_at_top(&stack)
}

type Stack = Vec<Vec<char>>;
type Move = (usize, usize, usize); // (move, from, to)

fn parse(input: &str) -> (Stack, Vec<Move>) {
    let (stacks, moves) = input.split_once("\n\n").unwrap();

    let mut stack_iter = stacks.lines().rev();

    let indices: Vec<usize> = stack_iter
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            if c.is_numeric() {
                return Some(i);
            }

            return None;
        })
        .collect();

    let mut stack: Vec<Vec<char>> = vec![vec![]; indices.len()];

    stack_iter.for_each(|l| {
        indices.iter().enumerate().for_each(|(stack_i, i)| {
            if *i <= l.len() - 1 {
                // input is ascii
                let c = l.as_bytes()[*i] as char;

                if !c.is_whitespace() {
                    stack[stack_i].push(c);
                }
            }
        });
    });

    let moves = moves
        .lines()
        .filter_map(|l| {
            let s: Vec<&str> = l.split_ascii_whitespace().collect();
            Some((s[1].parse().ok()?, s[3].parse().ok()?, s[5].parse().ok()?))
        })
        .collect();

    (stack, moves)
}

fn move_stacks(stack: &mut Stack, moves: &[Move], version: u16) {
    moves.iter().for_each(|(amount, from, to)| {
        let from = &mut stack[*from - 1];
        let crates = from.split_off(from.len() - amount);
        if version == 9000 {
            stack[*to - 1].extend(crates.iter().rev());
        } else {
            stack[*to - 1].extend(crates.iter());
        }
    });
}

fn crates_at_top(stacks: &Stack) -> String {
    stacks
        .iter()
        .filter_map(|stack| stack.last())
        .collect::<String>()
}
