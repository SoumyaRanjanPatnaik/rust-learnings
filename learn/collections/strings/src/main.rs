use std::stringify;
fn main() {
    // Create Empty String
    let mut _s = String::new();

    // Craete string from initial value
    // Method 1
    let _s = "Hello World!!".to_string();
    // Method 2
    let _s = String::from("Hello World again!!");

    // UTF-8 encoded, supports all properly encoded data directly 
    let s = String::from("Hello 😀");
    println!("unicode string: {s}\n");

    // Update a string
    // Push string slice
    let mut s = "Hello".to_string();
    println!("original string: {s}");
    s.push_str(" World");
    println!("Updated String: {s} \n");

    // Slice can be stored in variable
    let again = " again"; 
    s.push_str(again); // Slice borrowed and not moved
    println!("Updated String: {s} \n");
    println!("Variable again was not moved but borrowed as we can print it here: {again}");

    // Push character
    let emoji: char = '😆';
    s.push(emoji);
    println!("String after pushing emoji: {s}");

    // Concatenate 
    // Using plus (+) 
    let str1 = String::from("Soumya");
    let str2 = String::from("Ranjan");
    
    // Concatenate using plus requires the first left operand to be owned;
    let concat = str1 + " "+ &str2 ; // str1 moved into concat
    println!("String after concatenating to clone: {s}"); 
    //println!("str1: {str1}"); // Invalid as borrowing moved value

    // Concatenate without moving
    let concat2 = concat.clone() + " " + "Patnaik" ;
    println!("Concatenated without moving: {concat2}");
    println!("Original");

    // Using format macro to concatenate multiple strings
    let str1 = String::from( "tic" );
    let str2 = String::from( "tac" );
    let str3 = String::from( "toe" );
    let s = format!("{}-{}-{}",str1,str2,str3);
    println!("concatenated string is {s}");
    println!("ownership of variable not taken, for eg: str1: {str1}");

    // --------------------------------
    // Indexing into strings  gives error
    let s1 = String::from("Hello");
    //let h = s1[0]; // Invalid

    // Reason 1: each character can have different number of bytets
    println!("Length of {s1}: {}",s1.len()); // 5 bytes since character are ascii
    let s2 = String::from("नमस्ते");
    println!("Length of {s2} in hindi is: {}", s2.len()); // 19 bytes since each character is 3 bytes long

    // Reason 2: Several ways to represent string leading to ambiguity:
    // - 1: as byte array
    // - 2: as array of unicode scalar values(equivalent to char of other langs)
    // - 3: as grapheme clusters (diacritics are combined with letters)

    // Reason 3: To get the nth character, rust needs to check the size of each character giving
    // O(n) complexity.
    // --------------------------------


    // Slicing strings (Caution with UTF-8 chars)
    // Valid as 'H' takes only one byte
    println!("slice of s1 (0..1): {}", &s1[0..1]); 
    // Invalid as the first character has more than one bytes leading to invalid boundary
    //println!("slice of s2 (0..1): {}", &s2[0..1]);  // Invalid


    // Iterating string as characters
    for c in "नमस्ते".chars() {
        print!("{c} ");
    }
    //for c in s2.chars() { // Equivalent to above loop
        //print!("{c} ");
    //}
    println!("");

    // iterating string as bytes
    for c in "नमस्ते".bytes() {
        print!("{c} ");
    }
    println!("");

}
