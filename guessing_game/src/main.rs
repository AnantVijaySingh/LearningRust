use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..11);

    // println!("The secret number is: {}", secret_number);

    loop {

        println!("Please input your guess.");

        // The :: syntax in the ::new line indicates that new is an associated function of the String
        // type. An associated function is a function that’s implemented on a type, in this case String.
        // This new function creates a new, empty string.
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // & indicates that this argument is a reference
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // he underscore, _, is a catchall value; in this example, we’re
            // saying we want to match all Err values, no matter what information they have inside them.
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

    // println!("You guessed: {}", guess);
}
