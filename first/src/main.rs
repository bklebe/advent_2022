use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("no input.txt found!");
    let max = input.split("\n\n").map(|chunk| chunk.split("\n").map(|line| line.parse::<i32>().unwrap()).sum::<i32>()).max().unwrap();
    println!("{}", max);
}
