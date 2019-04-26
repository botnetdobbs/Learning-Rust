// Vectors - resizable arrays
use std::mem;

pub fn run() {
    let mut numbers: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8]; // Can be re-assigned but no additional element can be added

    println!("{:?}", numbers);
 
    // get single val
    println!("Single Value: {}", numbers[0]);

    // add on to a vector
    numbers.push(1000000000000);

    // pop off the last value
    // numbers.pop()

    // Vectors are stack allocated 
    println!("This vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i64] =  &numbers[0..2];
    println!("Slice: {:?}", slice);

    // loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop and mutate alues
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}