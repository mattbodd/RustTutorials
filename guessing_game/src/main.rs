// Standard input/output library brought into scope
use std::io;
// Rand is a non-standard crate that generates random numbers
use rand::Rng;
// Ordering is an enumeration with the possible variants: Less, Greater, Equal
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // The gen_range function in rang::Rng is inclusive on the lower bound and exclusive on the upper bound
    let secret_number = rand::thread_rng().gen_range(1,101);
    
    loop {
        println!("Please enter your guess.");
        
        // By default, variables are immutable in Rust so mut is used to make them mutable
        // The Rust string type is 'growable'
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Here we are shadowing the old value of guess
        // The u32 is a type annotation! Rust's type inference will infer secret_guess to be u32 too
        // The expect that was here is responding to the Result variant from .parse()
        // Instead of expect we can pattern match to avoid bad input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        // A match statement is made up of arms
        // An arm consists of a pattern and code to run if the value in the beginning of the match fits
        // the arm's pattern
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
