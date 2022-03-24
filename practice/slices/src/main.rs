fn main() {
    let mut arr  = [1,2,4,3];
    let mut arr_slice = &mut arr[1..3];
    println!("{:?}", arr_slice);
    arr_slice[0] = 10;
    println!("{:?}", arr_slice);
    arr_slice = &mut arr[1..];
    println!("{:?}", arr_slice);
    arr[0](2);
}
