use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("no input.txt found!");
    let calories = count_calories(&input);
    let max = calories.last().unwrap();
    println!("{}", max);
    let top3 = calories.iter().rev().take(3).sum::<i32>();
    println!("{}", top3);
}

fn count_calories(input: &String) -> Vec<i32> {
    let mut calories: Vec<i32> = input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split("\n")
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();
    calories.sort();
    calories
}
