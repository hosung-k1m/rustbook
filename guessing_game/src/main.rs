use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_num}");
    loop {
        println!("Please input your guess.");
        
        // let - create new variable
        // mut - set as mutable, immutable as default
        let mut guess = String::new(); // return a new instance of a String

        io::stdin()
            // pass mutable refrence to guess String, will append to the string
            .read_line(&mut guess)
            // read_line also returns a Result enum that has err or ok variants, must handle the error
            // on err or return val on ok
            .expect("Failed to read line");
        
        // can "shadow" the variable name to use again as an int
        let guess: u32 = match guess.trim().parse() {
            // match the enum value
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        
        // match the enum from cmp
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small :( "),
            Ordering::Greater => println!("Too large :( "),
            Ordering::Equal => {
                println!("You got it! :) ");
                break;
            } 
        }
    }

}
