use std::io;
// use rand::Rng;
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // In Rust, variables are immutable by default.
    // example,
    // let foo = 5; // immutable
    // let mut bar = 5; // mutable

    let mut guess = String::new();
    // The & indicates that this argument is a reference
    // references are immutable by default.
    // read_line will return io::Result ( enumerations type, Ok or Err )
    // If io::Result is Err value, then `expect` will be executed.
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
