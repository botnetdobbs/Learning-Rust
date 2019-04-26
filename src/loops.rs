// Loops - used to iterate until a condition is met.
 
pub fn run() {
    let mut count = 0;

    // // infinite loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);

    //     if count > 2000000 {
    //         break;
    //     }
    // }

    // // While loop (FizzBuzz)
    // while count <= 100 {
    //      if count % 15 == 0 {
    //           println!("FizzBuzz");
    //      } else if count % 3 == 0 {
    //           println!("fizz");
    //      }  else if  count % 5 == 0 {
    //          println!("buzz");
    //      } else {
    //          println!("{}", count);
    //      }

    //      //Increment
    //      count += 1;
    // }

    //For Range
    for x in 0..100 {
        if x % 15 == 0 {
              println!("FizzBuzz");
         } else if x % 3 == 0 {
              println!("fizz");
         }  else if  x % 5 == 0 {
             println!("buzz");
         } else { 
             println!("{}", x);
         }
    }
}