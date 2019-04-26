// Functions - used to store blocks of code for re-use

pub fn run() {
    greeting("Hello", "Lazarus");
    println!("Sum: {}", add(9, 9));

    // Closure
    let add_nums = |n1: i32, n2: i32| n1 + n2;

    println!("C Sum: {}", add_nums(9, 9));
}

fn greeting(greet: &str, name: &str) { 
    println!("{} {}. Nice to meet you.", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}