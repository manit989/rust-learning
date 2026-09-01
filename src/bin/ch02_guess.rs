use std::io;

fn main() {
    println!("Number guessing Game!");
    println!("Enter Your Guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Unable to take input");

    println!("Your guess is {guess}")
}
