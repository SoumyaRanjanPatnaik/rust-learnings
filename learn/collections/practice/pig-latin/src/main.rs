use std::io;
fn main() {
    let mut input = String::new();
    println!("Enter a word:");
    io::stdin().read_line(&mut input).unwrap(); 
    let input = input.trim();
    let first_char = input.chars()
        .nth(0)
        .unwrap()
        .to_lowercase()
        .next()
        .unwrap();
    let final_str;
    if !first_char.is_alphabetic() {
        println!("Not an english word: 🙃");
        return;
    }
    else if ['a', 'e', 'i', 'o', 'u'].contains(&first_char) {
        final_str = input.to_owned() + "-hay";
    }
    else {
        let suffix = format!("{first_char}{}","ay");
        final_str = format!("{}-{}",&input[1..], suffix );
    }
    println!("Welcome to pig latin 🙃...");
    println!("The pig latin for {} is {}", input, final_str);
}
