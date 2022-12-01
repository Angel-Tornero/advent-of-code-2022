use std::fs::read_to_string;

// Part one

/*
fn main() {
    let content = read_to_string("text/input.txt").unwrap();

    let result = content
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|number| number.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap();
    println!("The result is: {}", result);
}
*/

// Part two

fn main() {
    let content = read_to_string("text/input.txt").unwrap();

    let mut result: Vec<_> = content
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|number| number.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();
    result.sort_by(|a, b| b.cmp(a));
    println!("The result is: {}", result[0] + result[1] + result[2]);
}