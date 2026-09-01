use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Number guessing Game!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter Your Guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to take input");

        let guess: u32 = guess.trim().parse().expect("Please Type a number");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Correct! You win");
                break;
            }
        }
    }
}
