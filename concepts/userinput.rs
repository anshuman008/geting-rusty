// Import the standard input/output (I/O) library for user input handling.
use std::io;

fn main() {
    // Creating a mutable String to store the user's name input.
    // String in Rust is stored on the heap and can be dynamically sized.
    let mut name = String::new();

    // Asking the user for their name.
    println!("Please enter your name:");

    // Reading the user's input and storing it in the 'name' variable.
    // `read_line` takes a mutable reference to the variable and waits for the user to type input.
    // The `expect` method handles any errors that occur during input (e.g., if the input fails).
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    // Trimming the newline character from the input and printing the name.
    // The `trim` method is used to remove whitespace or newlines from both ends of a string.
    println!("Hello, {}!", name.trim());

    // Example: User input for guessing a number.
    let mut guess = String::new(); // String to store the user's guess.

    println!("Guess the number:");

    // Reading the user's guess.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    // Converting the guess from a String to an integer (u32).
    // The `trim` method removes the newline, and `parse` attempts to convert the string to a number.
    // `expect` will handle invalid input (e.g., if the input is not a valid number).
    let guess: u32 = guess.trim().parse().expect("Please enter a number!");

    // Checking if the guess is correct.
    if guess == 10 {
        println!("Your guess is correct!!");
    } else {
        println!("Your guess is not correct, sorry!");
    }
}
