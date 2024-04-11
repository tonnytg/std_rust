use std::io::{self, Write};

fn main() {
    println!("Enter text (press 'q' to quit):");

    loop {
        // Print a prompt
        print!("> ");
        io::stdout().flush().unwrap();

        // Create a mutable string to store user input
        let mut input = String::new();

        // Read user input from stdin
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Trim any leading or trailing whitespace
        let input = input.trim();

        // Check if the user wants to quit
        if input == "q" {
            println!("Exiting...");
            break; // Break out of the loop if 'q' is entered
        }

        // Print the user input
        println!("You entered: {}", input);
    }
}
