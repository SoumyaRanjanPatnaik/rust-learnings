enum WeekDay {
    WINT(i32),
    WText(String)
}
use WeekDay::*;
fn main() {
    let v: Vec<i32> = Vec::new(); // Explicit type
    println!("Created a new empty vector using explicit type declaration: {:?}",v);
    let v = vec![1,2,3]; // Implicit type inference
    println!("Created a vector {:?} using type inference", v);
    let e;
    println!("New Vector After pushing some data: {:?}", v);
    {
        let v_in_block = vec![1,2,3,4,5];
        println!("This vector(v_in_block) {:?} gets freed at the end of scope", v_in_block);
        e = &v_in_block[3];
        
        //do stuff  
    }// v goes out of scope
    //println!("{e}"); // Invalid as v_in_block doesn't live long enough

    // Updating vector
    let mut v = Vec::new();
    v.push(2);
    v.push(4);
    v.push(1);
    v.push(6);
    v.push(7);

    // Read elements of Vectors
    let v = vec![1,2,3,4,5];

    // Method 1: Borrowing a reference using square bracked syntax
    let third: &i32 = &v[2]; // reference
    println!("Third element reference: {third}");
    let third = v[2]; //copy
    println!("Third element copy: {third}");
    println!("Third element access using []: {}", v[2]);
    println!("The third element is: {third}");

    // Method 2: Using get to get a value of Option enum 
    match v.get(2) {
        Some(third) => println!("Third element using v.get(): {third}"),
        None => println!("No element at third index"),
    }

    // Method 2 is safer due to no ivalid index
    if let Some(x) = v.get(100) { // Can also use match instead of if let
        println!("v[100] is {}",  x);
    }
    else {
        println!("v[100] doesn't exist");
    };

    //let x = &v[100]; // Program will Panic
    //println!("v[100]: {}", x);

    // Refer to C++ example for better understanding
    let mut v = vec![5,6,7,8];
    let num = &v[2];
    v.push(6); // Not allowed if immutable reference in scope to prevent dangling reference in case of reallocation
    //println!("{}",num); // Keep num in scope (non lexical lifetime)


    // Iterate in vector
    let mut sum = 0;
    let x: u32 = 32;
    let y = 34-x;
    println!("{y}");
    for i in &v {
        sum += i;
    }
    println!("Sum of nums in vector {:?} is {sum}",v);

    let v = [WText(String::from("Monday")), WINT(2)]; // Valid, can store multiple types
    
    // Types that don't implement copy trait are moved into the vector
    let s = String::from("Hello World"); 
    let str_vec = vec![s];
    //println!("s is still accessible: {s}"); // Invalid as str_vec owns the data
    // let x = str_vec[0]; // Cannot move the value out of the vector
}



