use avg::average;
use std::io;

fn _get_int_input() ->i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid Input");
    let num:i32 =  match input.trim().parse() {
        Ok(n) => n,
        Err(_) => 1,
    };
    num
}

fn get_vec_input() -> Vec<i32> {
    let v;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error Encountered...");
    v = input.trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Invalid Vector..."))
        .collect();
    v
}

fn main() {
    println!("Enter the elements in the vector: ");
    let v = get_vec_input();
    println!("Mean: {}", average::mean(&v));
    println!("Median: {}", average::median(&v));
    println!("Mode: {:?}", average::mode(&v));
}
