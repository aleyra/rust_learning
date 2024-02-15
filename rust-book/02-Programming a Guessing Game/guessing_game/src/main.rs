use std::io; // standard lib
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!"); // println! is a macro that prints a string to the screen

    let secret_number = rand::thread_rng().gen_range(1..=100); // variable immutable
    
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new(); // variable mutable
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failes to read line"); // to handle potential failure with result
    
        // let guess: u32 = guess.trim().parse().expect("Please type a number!"); // Rust allows us to shadow the previous value of guess with a new one.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}"); // {} in println! is a placeholder. Other possibility : println!("you guessed: {}", guess);
    
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
