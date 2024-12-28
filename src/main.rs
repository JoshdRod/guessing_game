use std::io;
use std::cmp::Ordering; // We need to bring the 'ordering' enum into scope (?)
use rand::Rng;

fn main() {
    // Print "Guess"
    println!("Enter a number (1 - 100)");

    // Create a secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");

    loop {
        // Take in guess
        let mut guess = String::new(); // Guess is mutable, bc its size can change
        io::stdin()
            .read_line(&mut guess) // Output of read_line put in guess
            .expect("Couldn't parse input"); // Outputs error message when an error type is returned from stdin().read_line() (There was some sort of error reading the line)
       
        // Cast it to an int
        let guess: u32 = match guess.trim().parse() { // The value of guess is set to the return value of the match statement
            Ok(num) => num, // If guess.trim().parse() returned an Ok Result variant (converting str -> int was successful), return the casted int
            Err(_) => { // If casting was unsuccessful, go to next iteration of loop (continue;)
                println!("Please Type a number!");
                continue;
            }
        };

        // Compare guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
