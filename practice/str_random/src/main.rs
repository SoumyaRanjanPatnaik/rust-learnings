fn main() {
    let s = String::from("Здравствуйте");
    let third = &s[2..3].chars().next().unwrap();
    println!("{third}");
}
