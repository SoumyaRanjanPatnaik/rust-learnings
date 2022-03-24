// Bring HashMap into scope
use std::collections::HashMap;

use std::fmt::Result;
// use std::io::Result; // Error (Naming conflict)

// Resolved by either not bringing conflicting members or submodules into scope or 
// by assigning new name to the one of the member or submodule using the 'as' keyword
use std::io::Result as IoResult;
fn main() {
    let mut map = HashMap::<i32,i32>::new();
    map.insert(1, 2);
}
