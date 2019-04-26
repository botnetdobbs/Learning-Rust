pub fn run() {
    let check_id: bool = true;
    let knows_person_of_age = true;
    let age = 18;

    if age >= 21 {
        println!("Bartender: What would you like to drink?");
    } else if age > 21 && check_id || knows_person_of_age{
        println!("Bartender: Sorry, you have to leave.");
    } else {
        println!("Bartender: I'll need to see your ID.");
    }

    // shorthand if
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of Age: {}", is_of_age);
}