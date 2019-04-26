// Arrays - Fixed lists where elements are of the same data types
use std::mem;

pub fn run() {
    let numbers: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8]; // Can be re-assigned but no additional element can be added

    println!("{:?}", numbers);
 
    // get single val
    println!("Single Value: {}", numbers[0]);

    // arrays are stack allocated
    println!("This array occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] =  &numbers[0..2];
    println!("Slice: {:?}", slice);
}