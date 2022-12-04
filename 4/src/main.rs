use std::fs::read_to_string;
use std::ops::RangeInclusive;

fn main() {
    let content = read_to_string("text/input.txt").unwrap();
    println!("The result for part 1 is: {}", first_star(content.clone()));
    println!("The result for part 2 is: {}", second_star(content));
}

// Part one

fn first_star(content: String) -> usize {
    content
        .split('\n')
        .map(|row| row.split(','))
        .map(|pair| {
            pair.map(|section| {
                section
                    .split('-')
                    .map(|str_number| str_number.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .map(|section| vector_to_range_inclusive(section))
            .collect::<Vec<_>>()
        })
        .filter(|range_pair| {
            fully_contains(&range_pair[0], &range_pair[1])
                || fully_contains(&range_pair[1], &range_pair[0])
        })
        .count()
}

fn vector_to_range_inclusive(vector: Vec<usize>) -> RangeInclusive<usize> {
    vector[0]..=vector[1]
}

fn fully_contains(range_a: &RangeInclusive<usize>, range_b: &RangeInclusive<usize>) -> bool {
    range_b.contains(range_a.start()) && range_b.contains(range_a.end())
}

// Part two

fn second_star(content: String) -> usize {
    content
        .split('\n')
        .map(|row| row.split(','))
        .map(|pair| {
            pair.map(|section| {
                section
                    .split('-')
                    .map(|str_number| str_number.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .map(|section| vector_to_range_inclusive(section))
            .collect::<Vec<_>>()
        })
        .filter(|range_pair| exist_overlapped_range(&range_pair[0], &range_pair[1]))
        .count()
}

fn exist_overlapped_range(
    range_a: &RangeInclusive<usize>,
    range_b: &RangeInclusive<usize>,
) -> bool {
    if range_a.contains(range_b.start())
        || range_b.contains(range_a.start())
        || range_a.contains(range_b.end())
        || range_b.contains(range_a.end())
    {
        return true;
    }
    return false;
}
