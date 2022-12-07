use std::fs::read_to_string;

fn main() {
    let content = read_to_string("text/input.txt").unwrap();
    println!("The result for part 1 is: {}", first_star(content.clone()));
    println!("The result for part 2 is: {}", second_star(content));
}

// Part one

fn first_star(content: String) -> usize {
    let datastream_buffer = content.chars().collect::<Vec<_>>();
    'outer: for index in 3..datastream_buffer.len() {
        let possible_marker = &datastream_buffer[index - 3..=index];
        for i in 0..possible_marker.len() {
            for j in (i + 1)..possible_marker.len() {
                if possible_marker[i] == possible_marker[j] {
                    continue 'outer;
                }
            }
        }
        return index + 1;
    }
    return 0;
}

// Part two

fn second_star(content: String) -> usize {
    let datastream_buffer = content.chars().collect::<Vec<_>>();
    'outer: for index in 13..datastream_buffer.len() {
        let possible_marker = &datastream_buffer[index - 13..=index];
        for i in 0..possible_marker.len() {
            for j in (i + 1)..possible_marker.len() {
                if possible_marker[i] == possible_marker[j] {
                    continue 'outer;
                }
            }
        }
        return index + 1;
    }
    return 0;
}
