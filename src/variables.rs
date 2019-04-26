//Variables holdprimitive data or references to data
//Rust variables are immutable by default
//Blocked scoped

pub fn run() {
     let name = "Lazarus";
    //  name = "Lazarus update"; This won't work {Immutability}
    // let mut name = "Lazarus"; This is now mutable 
     println!("My name is {}", name);

    // Define constants
    const ID: i32 =  001;
    println!("ID: {}", ID);

    // Assign multiple variables

    let (my_name, my_age) = ("Lazarus", "22");
    println!("{} is {}", my_name, my_age);
}