use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let content = read_to_string("text/input.txt").unwrap();
    println!("The result for part 1 is: {}", first_star(content.clone()));
    println!("The result part 2 is: {}", second_star(content));
}

// Part one

fn first_star(content: String) -> i32 {
    let mut move_value = HashMap::<&str, i32>::new();
    move_value.insert("X", 1);
    move_value.insert("Y", 2);
    move_value.insert("Z", 3);
    let mut game_score = HashMap::<(&str, &str), i32>::new();
    game_score.insert(("A", "X"), 3);
    game_score.insert(("A", "Y"), 6);
    game_score.insert(("A", "Z"), 0);
    game_score.insert(("B", "X"), 0);
    game_score.insert(("B", "Y"), 3);
    game_score.insert(("B", "Z"), 6);
    game_score.insert(("C", "X"), 6);
    game_score.insert(("C", "Y"), 0);
    game_score.insert(("C", "Z"), 3);

    let result: i32 = content
        .split("\n")
        .map(|round| round.split(" ").collect::<Vec<_>>())
        .map(|play| game_score.get(&(play[0], play[1])).unwrap() +
             move_value.get(play[1]).unwrap())
        .sum::<i32>();

    return result;
}

// Part two

fn second_star(content: String) -> i32 {
    let mut game_score = HashMap::<(&str, &str), i32>::new();
    game_score.insert(("A", "X"), 3);  // you need scizor (3 pts) to lose (0 pts)
    game_score.insert(("A", "Y"), 4);  // you need rock (1 pt) to draw (3 pts)
    game_score.insert(("A", "Z"), 8);  // you need paper (2 pts) to win (6 pts)
    game_score.insert(("B", "X"), 1);  // you need rock (1 pt) to lose (0 pts)
    game_score.insert(("B", "Y"), 5);  // you need paper (2 pts) to draw (3 pts)
    game_score.insert(("B", "Z"), 9);  // you need scizor (3 pts) to win (6 pts)
    game_score.insert(("C", "X"), 2);  // you need paper (2 pts) to lose (0 pts)
    game_score.insert(("C", "Y"), 6);  // you need scizor (3 pts) to draw (3 pts)
    game_score.insert(("C", "Z"), 7);  // you need rock (1 pt) to win (6 pts)

    let result: i32 = content
        .split("\n")
        .map(|round| round.split(" ").collect::<Vec<_>>())
        .map(|play| game_score.get(&(play[0], play[1])).unwrap())
        .sum::<i32>();

    return result;
}