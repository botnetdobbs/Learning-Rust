// Enums - types which have a few definite values

enum Movement {
    // variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    // perform action depending on info
    //Match resembles a switch
    match m {
       Movement::Up => println!("Avatar moving up"),
       Movement::Down => println!("Avatar moving down"), 
       Movement::Left => println!("Avatar moving left"), 
       Movement::Right => println!("Avatar moving right") 
    }
}

pub fn run() {
    let avatar  = Movement::Up;
    let avatar1 = Movement::Down;
    let avatar2 = Movement::Left;
    let avatar3 = Movement::Right;

    move_avatar(avatar);
}