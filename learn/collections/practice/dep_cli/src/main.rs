use std::{collections::HashMap, io::{self, Write}, error::Error};

fn get_char_str(ch: char,size: usize) -> String {
    let mut spaces = String::new();
    for _ in 0..size {
        spaces.push(ch);
    }
    spaces
}

fn add_to_dep(deps: &mut HashMap<String, Vec<String>>, tokens: &Vec<&str>) {
    let key_val: Vec<String> = tokens
        .split(|&s| s == "to")
        .map(|arr| arr.join(" "))
        .collect();
    let person = &key_val[1..].join(" ");
    let key_val = vec![&key_val[0], person];
    if key_val.len() != 2 {
        panic!("Invalid Syntax");
    }
    let people_ptr = deps.entry(String::from(key_val[1])).or_insert(Vec::new());
    people_ptr.push(key_val[0].clone());
    if people_ptr.len() > 1 {
        println!("Department Created: {}",key_val[1]);
    }
    else {
        println!("Transaction Succeeded!");
    }
}
fn select(deps: &HashMap<String, Vec<String>>, tokens: &Vec<&str>)
    ->Result<(), Box<dyn Error>> {
    let mut max_len = "department".len();
    let mut max_len2 = "person".len();
    let tokens: Vec<String>= tokens
        .split(|&s| s == ",")
        .map(|arr| arr.join(" "))
        .collect();
    dbg!("{}", &tokens);
    for s in &tokens {
        if s.len() > max_len {
            max_len = s.len();
        }
        let mut len = 0;
        for s_vec in deps.get(s) {
            for s in s_vec {
                len += s.len();
            }
        }
        if len > max_len2 {
            max_len2 = len;
        }
    };
    let sep = format!("+{}+",get_char_str('-', max_len + max_len2 + 3));
    println!("{sep}");

    println!("|{}{} |{}{} |", 
             "Department",
             get_char_str(' ', max_len-"department".len()),
             "Person",get_char_str( ' ', max_len2-"Person".len() ));

    println!("{sep}");

    for s in &tokens {
        for p in deps.get(s).ok_or("Key Error")? {
            println!("|{}{} |{}{} |", s,
                     get_char_str( ' ',max_len-s.len() ),
                     p,get_char_str(' ', max_len2-p.len() ));
        }
    };
    println!("{sep}");
    Ok(())
}
fn main() {
    let mut departments:HashMap<String, Vec<String>> = HashMap::new();
    let mut input = String::new();
    loop {
        print!("#> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input = input.to_lowercase();
        let tokens:Vec<&str> = input
            .trim()
            .split_whitespace()
            .collect();
        let query = tokens[0];
        let tokens = tokens[1..].to_owned();
        match query {
            "help" => println!("Ask Soumya"),
            "show" => println!("{:?}", departments),
            "add" => add_to_dep(&mut departments, &tokens),
            "select" => if let Err(e) = select(&departments, &tokens) {
                println!("Error: {e}");
            },
            "quit" => break,
            "exit" => break,
            _ => println!("Invalid syntax"),
        }
        input.clear();
    }
    println!("Thank you 😀");
}
