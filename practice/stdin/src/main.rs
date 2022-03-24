use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error Occurred");
    let int_val;
    match input.trim().parse::<i32>() {
        Ok(val) => int_val = val,
        Err(_) => int_val = 0,
    };
}
