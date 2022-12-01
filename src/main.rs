use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    
    let mut max_calories_carried: u64 = 0;
    let mut calories_carried: u64 = 0;

    for line in stdin.lock().lines().map(Result::unwrap) {
        if line.trim().is_empty() {
            if calories_carried > max_calories_carried {
                max_calories_carried = calories_carried;
            }
            calories_carried = 0;
        } else {
            calories_carried += line.parse::<u64>().unwrap();
        }
    }
    if calories_carried > max_calories_carried {
        max_calories_carried = calories_carried;
    }
    
    println!("{}", max_calories_carried);
}
