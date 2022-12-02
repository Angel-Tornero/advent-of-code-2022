use std::fs::read_to_string;

fn main() {
    let content = read_to_string("text/input.txt").unwrap();
    println!("The result for part 1 is: {}", first_star(content.clone()));
    println!("The result part 2 is: {}", second_star(content));
}

// Part one

fn first_star(content: String) -> usize {
    let result = content
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|number| number.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap();
    return result;
}

// Part two

fn second_star(content: String) -> usize {
  let mut result: Vec<_> = content
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|number| number.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();
    result.sort_by(|a, b| b.cmp(a));
    return result[0] + result[1] + result[2];
    // return result.iter().take(3).sum::<usize>().unwrap();
}
