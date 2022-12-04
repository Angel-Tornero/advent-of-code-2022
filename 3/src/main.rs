use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let content = read_to_string("text/input.txt").unwrap();
    println!("The result for part 1 is: {}", first_star(content.clone()));
    println!("The result for part 2 is: {}", second_star(content));
}

// Part one

fn first_star(content: String) -> usize {
    content
        .split('\n')
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .map(|compartment| get_shared_char(&[compartment.0, compartment.1]).unwrap())
        .map(|shared_char| get_char_priority(&shared_char))
        .sum::<usize>()
}

fn get_char_priority(character: &char) -> usize {
    let ascii_code: usize = *character as usize;
    if ascii_code >= 'a' as usize && ascii_code <= 'z' as usize {
        return ascii_code as usize - 'a' as usize + 1;
    }
    return ascii_code as usize - 'A' as usize + 27;
}

// Part two

fn second_star(content: String) -> usize {
    content
        .split('\n')
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| get_shared_char(group).unwrap())
        .map(|badge| get_char_priority(&badge))
        .sum::<usize>()
}

fn get_shared_char(strings_group: &[&str]) -> Option<char> {
    let mut items = HashMap::<char, usize>::new();
    for character in strings_group[0].chars() {
        items.insert(character, 1);
    }
    for index in 1..strings_group.len() {
        for character in strings_group[index].chars() {
            let item = items.get(&character).unwrap_or_else(|| &0).clone();
            if item != index {
                continue;
            }
            if item == strings_group.len() - 1 {
                return Some(character);
            }
            items.insert(character, item + 1);
        }
    }
    None
}