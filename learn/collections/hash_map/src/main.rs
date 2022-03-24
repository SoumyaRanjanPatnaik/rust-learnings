use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 50);
    scores.insert(String::from("Green"), 80);
    println!("Scores of teams are: {:?}",scores);


    // Constructing HashMap using two vectors or one vector of tuples
    // Method 1: two vectors
    let teams = vec!['A', 'B', 'V', 'D'];
    let ids = vec![12,543,25,765];
    let teams_with_ids:HashMap<_,_> = teams.into_iter().zip(ids.into_iter()).collect();
    println!("teams with ids: {:?}",teams_with_ids);

    // Method 2: vector of tuples 
    let teams_with_ids_vec = vec![('A',1), ('B',12), ('C',14)];
    let teams_with_ids: HashMap<_,_> = teams_with_ids_vec.into_iter().collect();
    println!("teams_with_ids map from vector of tuples: {:?}",teams_with_ids);

    // Ownership transferred if references not used
    let key = String::from("Color");
    let val = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(key, val);
    println!("Elemnets moved into map: {:?}", map);
    //println!("Cannot access key {key} or val {val}"); // Invalid as the value has been moved into hash map

    // Read values
    let team = "Blue";
    let score = scores.get(team); // Returns value wrapped in Option
    if let Some(x) = score {
        println!("Score for {team}: {x}");
    }
    else {
        println!("Invalid key...");
    }

    // Overwriting value for key in HashMap 
    scores.insert(String::from("Blue"), 0);
    println!("scores after overwritng value of \"Blue\": {:?}", scores);

    // Insert only if key has no value 
    scores.entry(String::from("Yellow")).or_insert(25); // Yellow inserted as it doesn't exists
    scores.entry(String::from("Blue")).or_insert(25); // Blue not updated as it already exists
    println!("scores after overwritng value of \"Blue\": {:?}", scores);

    // Update based on old value

    let text = "hello world 😉!!!";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // or_insert returns a mutable reference
        *count += 1; //dereference count and add one
    }
    println!("{:?}", map);
}
