pub fn run() {
    // Print to console
    println!("This is the print.rs file.");

    // Formatting
    println!("This is number: {}", 1);
    println!("{} Is my birthday, {} is the month and {} is the year", 28, "September", 1996);

    // Positional arguments
    println!("{0} like {1} but {0} like lazying around too", "I", "working out"); // Using "I" twice

     // Named arguments
     println!("{name} liked to play {activity} in high school", name = "Lazarus", activity = "Cricket");

     // Placeholder traits
     println!("Binary: {:b} Hex: {:x} Octal: {:o }", 10, 10, 10);

     // Placeholder for debug trait
     println!("{:?}", (12, true, "Hello"));
} 