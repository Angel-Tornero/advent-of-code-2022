use std::collections::HashMap;
use std::fs::read_to_string;

struct CargoInstruction {
    crates_to_move: usize,
    from: char,
    to: char,
}

impl CargoInstruction {
    pub fn new(crates_to_move: usize, from: char, to: char) -> Self {
        Self {
            crates_to_move,
            from,
            to,
        }
    }

    pub fn from(&self) -> char {
        self.from.clone()
    }

    pub fn to(&self) -> char {
        self.to.clone()
    }

    pub fn crates_to_move(&self) -> usize {
        self.crates_to_move
    }
}

fn main() {
    let content = read_to_string("text/input.txt").unwrap();
    println!("The result for part 1 is: {}", first_star(content.clone()));
    println!("The result for part 2 is: {}", second_star(content));
}

// Part one

fn first_star(content: String) -> String {
    let input_parts = content.split("\n\n").collect::<Vec<_>>();
    let mut starting_stacks_part = input_parts[0]
        .split('\n')
        .map(|stack_level| stack_level.trim())
        .collect::<Vec<_>>();
    let stack_tags = starting_stacks_part
        .pop()
        .unwrap()
        .chars()
        .filter(|tag| *tag != ' ')
        .collect::<Vec<_>>();
    let stack_distribution = starting_stacks_part
        .into_iter()
        .map(|stack_level| split_by_length(stack_level, 4))
        .collect::<Vec<_>>();
    let mut stacks = HashMap::<char, Vec<String>>::new();
    for tag in stack_tags.clone() {
        stacks.insert(tag, Vec::<String>::new());
    }

    for stack_row in stack_distribution.into_iter().rev() {
        for index in 0..stack_row.len() {
            let stack_crate = stack_row[index]
                .trim_matches(&['[', ']', ' '] as &[_])
                .to_owned();
            if !stack_crate.is_empty() {
                stacks
                    .get_mut(&stack_tags[index])
                    .unwrap()
                    .push(stack_crate);
            }
        }
    }

    let rearrangement_procedure = input_parts[1]
        .split('\n')
        .map(|instruction| instruction.split(' ').collect::<Vec<_>>())
        .map(|movement| {
            CargoInstruction::new(
                movement[1].parse::<usize>().unwrap(),
                movement[3].chars().last().unwrap(),
                movement[5].chars().last().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    for instruction in rearrangement_procedure {
        for _ in 0..instruction.crates_to_move() {
            let crate_to_move = stacks.get_mut(&instruction.from()).unwrap().pop().unwrap();
            stacks
                .get_mut(&instruction.to())
                .unwrap()
                .push(crate_to_move);
        }
    }
    let mut result = "".to_owned();
    for tag in stack_tags {
        result += stacks.get(&tag).unwrap().last().unwrap();
    }
    return result;
}

fn split_by_length(string: &str, number: usize) -> Vec<&str> {
    string
        .as_bytes()
        .chunks(number)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
}

// Part two

fn second_star(content: String) -> String {
    let input_parts = content.split("\n\n").collect::<Vec<_>>();
    let mut starting_stacks_part = input_parts[0]
        .split('\n')
        .map(|stack_level| stack_level.trim())
        .collect::<Vec<_>>();
    let stack_tags = starting_stacks_part
        .pop()
        .unwrap()
        .chars()
        .filter(|tag| *tag != ' ')
        .collect::<Vec<_>>();
    let stack_distribution = starting_stacks_part
        .into_iter()
        .map(|stack_level| split_by_length(stack_level, 4))
        .collect::<Vec<_>>();
    let mut stacks = HashMap::<char, Vec<String>>::new();
    for tag in stack_tags.clone() {
        stacks.insert(tag, Vec::<String>::new());
    }

    for stack_row in stack_distribution.into_iter().rev() {
        for index in 0..stack_row.len() {
            let stack_crate = stack_row[index]
                .trim_matches(&['[', ']', ' '] as &[_])
                .to_owned();
            if !stack_crate.is_empty() {
                stacks
                    .get_mut(&stack_tags[index])
                    .unwrap()
                    .push(stack_crate);
            }
        }
    }

    let rearrangement_procedure = input_parts[1]
        .split('\n')
        .map(|instruction| instruction.split(' ').collect::<Vec<_>>())
        .map(|movement| {
            CargoInstruction::new(
                movement[1].parse::<usize>().unwrap(),
                movement[3].chars().last().unwrap(),
                movement[5].chars().last().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    for instruction in rearrangement_procedure {
        let mut moving_stack = Vec::<String>::new();
        for _ in 0..instruction.crates_to_move() {
            let crate_to_move = stacks.get_mut(&instruction.from()).unwrap().pop().unwrap();
            moving_stack.push(crate_to_move);
        }
        for moving_crate in moving_stack.into_iter().rev() {
            stacks
                .get_mut(&instruction.to())
                .unwrap()
                .push(moving_crate);
        }
    }
    let mut result = "".to_owned();
    for tag in stack_tags {
        result += stacks.get(&tag).unwrap().last().unwrap();
    }
    return result;
}
