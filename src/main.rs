use std::io::{self, BufRead};
use std::collections::BinaryHeap;

fn main() {
    let stdin = io::stdin();
    
    let mut sorted_calories = BinaryHeap::new();
    let mut calories_carried: u64 = 0;

    for line in stdin.lock().lines().map(Result::unwrap) {
        if line.trim().is_empty() {
            sorted_calories.push(calories_carried);
            calories_carried = 0;
        } else {
            calories_carried += line.parse::<u64>().unwrap();
        }
    }
    sorted_calories.push(calories_carried);
    
    let max_calories_carried = sorted_calories.pop().unwrap();
    println!("{}", max_calories_carried);
    println!("{}", max_calories_carried + sorted_calories.pop().unwrap() + sorted_calories.pop().unwrap());
}
