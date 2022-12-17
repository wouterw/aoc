use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");

    let moves: Vec<(char, char)> = input
        .lines()
        .map(|s| {
            let chars: Vec<char> = s.chars().filter(|c| !c.is_whitespace()).collect();
            (chars[0], chars[1])
        })
        .collect();

    let total_score = total_score_according_to_strategy(&moves);
    let total_score_2 = total_score_according_to_strategy_2(&moves);

    println!("Part 1: {:?}", total_score);
    println!("Part 2: {:?}", total_score_2);
}

fn total_score_according_to_strategy(moves: &Vec<(char, char)>) -> usize {
    let mut score: usize = 0;
    let points_per_move: HashMap<char, usize> = HashMap::from([('X', 1), ('Y', 2), ('Z', 3)]);

    for m in moves {
        // Opponent plays ROCK
        if m.0 == 'A' {
            // ROCK
            if m.1 == 'X' {
                score += points_per_move.get(&m.1).unwrap() + 3;
            }

            // PAPER
            if m.1 == 'Y' {
                score += points_per_move.get(&m.1).unwrap() + 6;
            }

            // SCISSORS
            if m.1 == 'Z' {
                score += points_per_move.get(&m.1).unwrap();
            }
        }

        // Opponent plays PAPER
        if m.0 == 'B' {
            // ROCK
            if m.1 == 'X' {
                score += points_per_move.get(&m.1).unwrap();
            }

            // PAPER
            if m.1 == 'Y' {
                score += points_per_move.get(&m.1).unwrap() + 3;
            }

            // SCISSORS
            if m.1 == 'Z' {
                score += points_per_move.get(&m.1).unwrap() + 6;
            }
        }

        // Opponent plays SCISSORS
        if m.0 == 'C' {
            // ROCK
            if m.1 == 'X' {
                score += points_per_move.get(&m.1).unwrap() + 6;
            }

            // PAPER
            if m.1 == 'Y' {
                score += points_per_move.get(&m.1).unwrap();
            }

            // SCISSORS
            if m.1 == 'Z' {
                score += points_per_move.get(&m.1).unwrap() + 3;
            }
        }
    }

    score
}

fn total_score_according_to_strategy_2(moves: &Vec<(char, char)>) -> usize {
    let mut score: usize = 0;
    let points_per_move: HashMap<char, usize> = HashMap::from([('A', 1), ('B', 2), ('C', 3)]);

    for m in moves {
        // You need to loss
        if m.1 == 'X' {
            match m.0 {
                // ROCK
                'A' => score += points_per_move.get(&'C').unwrap(),
                // PAPER
                'B' => score += points_per_move.get(&'A').unwrap(),
                // SCISSORS
                'C' => score += points_per_move.get(&'B').unwrap(),
                _ => println!("derp"),
            }
        }

        // You need to draw
        if m.1 == 'Y' {
            match m.0 {
                // ROCK
                'A' => score += points_per_move.get(&'A').unwrap() + 3,
                // PAPER
                'B' => score += points_per_move.get(&'B').unwrap() + 3,
                // SCISSORS
                'C' => score += points_per_move.get(&'C').unwrap() + 3,
                _ => println!("derp"),
            }
        }

        // You need to win
        if m.1 == 'Z' {
            match m.0 {
                // ROCK
                'A' => score += points_per_move.get(&'B').unwrap() + 6,
                // PAPER
                'B' => score += points_per_move.get(&'C').unwrap() + 6,
                // SCISSORS
                'C' => score += points_per_move.get(&'A').unwrap() + 6,
                _ => println!("derp"),
            }
        }
    }

    score
}
