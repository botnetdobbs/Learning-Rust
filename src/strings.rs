// Primitive Str - Immutable fixed length string somewhere in memory
// String - Growable, heap-allocated data structure - Used when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("Hello ");
    println!("{}", hello);
    // get length
    println!("Length: {}", hello.len());

    hello.push('W'); // push in a unicode character

    println!("{}", hello);

    hello.push_str("orld"); // push in a string
 
    println!("{}", hello);

    // capacity in bytes
    println!("Capacity: {} bytes", hello.capacity());
    println!("Is Empty?: {}", hello.is_empty());
    println!("Contains Hello?: {}", hello.contains("Hello"));
    println!("{}", hello.replace("World", "There"));

    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create a string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertions
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity()); 
}