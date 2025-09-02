use std::io::{self, Write};

// Define a simple struct to demonstrate ownership
struct Greeter {
    name: String,
    greeting_count: u32,
}

impl Greeter {
    // Constructor function
    fn new(name: String) -> Self {
        Greeter {
            name,
            greeting_count: 0,
        }
    }

    // Method that takes ownership of self
    fn greet(&mut self) -> String {
        self.greeting_count += 1;
        format!("Hello, {}! This is greeting #{}.", self.name, self.greeting_count)
    }

    // Method that borrows self immutably
    fn get_info(&self) -> String {
        format!("Name: {}, Total greetings: {}", self.name, self.greeting_count)
    }
}

fn main() {
    println!("Welcome to Rust Beginner Toolkit!");
    println!("==================================");
    
    // Get user input
    print!("Enter your name: ");
    io::stdout().flush().unwrap(); // Ensure prompt is displayed
    
    let mut input = String::new();
    
    // Handle potential errors with Result
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Trim whitespace and newlines
            let name = input.trim().to_string();
            
            // Create a Greeter instance (demonstrates ownership)
            let mut greeter = Greeter::new(name);
            
            // Demonstrate multiple method calls
            println!("{}", greeter.greet());
            println!("{}", greeter.greet());
            println!("{}", greeter.get_info());
            
            // Show ownership transfer
            let name_owned = greeter.name; // This takes ownership
            println!("Name extracted: {}", name_owned);
            
            // This would cause a compilation error:
            // println!("{}", greeter.name); // Error: value borrowed here after move
        }
        Err(error) => {
            eprintln!("Error reading input: {}", error);
        }
    }
}
