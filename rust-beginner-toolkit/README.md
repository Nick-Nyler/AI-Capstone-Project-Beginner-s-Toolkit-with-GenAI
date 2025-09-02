# Getting Started with Rust – A Beginner's Toolkit

## 🎯 Title & Objective

**"Getting Started with Rust – A Beginner's Guide"**

### What technology did you choose?
I chose **Rust** - a modern systems programming language that offers memory safety without garbage collection.

### Why did you choose it?
- Rust is rapidly growing in popularity and adoption
- It offers unique memory safety guarantees
- It's being used in major projects like Linux kernel, WebAssembly, and blockchain
- Perfect for learning systems programming concepts

### What's the end goal?
Create a simple CLI application that demonstrates Rust's core features: ownership, borrowing, and error handling.

## 📋 Quick Summary of the Technology

### What is Rust?
Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. It achieves memory safety without garbage collection through its unique ownership system.

### Where is it used?
- Systems programming (operating systems, embedded systems)
- Web development (WebAssembly)
- Blockchain and cryptocurrency
- Game development
- Command-line tools and utilities

### One real-world example:
Mozilla Firefox uses Rust for performance-critical components, and the Linux kernel now accepts Rust code contributions.

## 💻 System Requirements

### OS: 
- Linux (Ubuntu 20.04+)
- macOS (10.15+)
- Windows (10+)

### Tools/Editors required:
- VS Code with Rust extension
- Terminal/Command Prompt
- Git

### Packages:
- Rust toolchain (rustc, cargo)

## 🚀 Installation & Setup Instructions

### Step 1: Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Step 2: Verify Installation
```bash
rustc --version
cargo --version
```

### Step 3: Create Your First Project
```bash
cargo new hello_rust
cd hello_rust
```

## 🎯 Minimal Working Example

### What the example does:
A simple CLI tool that takes user input, processes it using Rust's ownership system, and demonstrates error handling.

### Code with inline comments:
```rust
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
```

### Expected output:
```
Welcome to Rust Beginner Toolkit!
==================================
Enter your name: Alice
Hello, Alice! This is greeting #1.
Hello, Alice! This is greeting #2.
Name: Alice, Total greetings: 2
Name extracted: Alice
```

## 🤖 AI Prompt Journal

### Prompt 1: "Give me a step-by-step guide to set up a Rust development environment and create my first program"

**Link to curriculum:** [AI Learning Platform - Rust Setup]

**AI's response summary:** The AI provided a comprehensive setup guide including rustup installation, cargo project creation, and basic syntax explanation.

**Your evaluation:** Extremely helpful - the AI broke down the installation process into digestible steps and explained each command's purpose.

### Prompt 2: "Explain Rust's ownership system with practical examples that demonstrate borrowing and moving"

**Link to curriculum:** [AI Learning Platform - Rust Ownership]

**AI's response summary:** The AI created examples showing how ownership prevents data races and memory issues, with clear explanations of borrowing rules.

**Your evaluation:** Very helpful - the examples made abstract concepts concrete and showed common pitfalls.

### Prompt 3: "Help me create a simple CLI application that demonstrates Rust's error handling with Result and Option types"

**Link to curriculum:** [AI Learning Platform - Rust Error Handling]

**AI's response summary:** The AI showed how to use match expressions and the ? operator for elegant error handling.

**Your evaluation:** Excellent - this made error handling feel natural and safe rather than cumbersome.

## 🐛 Common Issues & Fixes

### Issue 1: "Cannot find `rustc` command"
**Error:** `command not found: rustc`
**Solution:** 
```bash
source ~/.cargo/env
# or restart your terminal
```

### Issue 2: "Value borrowed here after move"
**Error:** `error[E0382]: use of moved value`
**Solution:** Use references (`&`) or clone the data:
```rust
let borrowed_name = &greeter.name; // Borrow instead of move
```

### Issue 3: "Expected `String`, found `&str`"
**Error:** Type mismatch between string types
**Solution:** Convert using `.to_string()` or `.to_owned()`:
```rust
let name: String = "Alice".to_string();
```

### Issue 4: "Cannot borrow as mutable"
**Error:** `error[E0596]: cannot borrow as mutable`
**Solution:** Declare variable as mutable:
```rust
let mut greeter = Greeter::new(name);
```

## 📚 References

### Official Documentation:
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Reference](https://doc.rust-lang.org/reference/)

### Video Resources:
- [Rust Crash Course](https://www.youtube.com/watch?v=zF34dJivmeE)
- [Rust Ownership Explained](https://www.youtube.com/watch?v=VFIOSWy93H0)

### Helpful Blog Posts:
- [Rust Ownership Rules](https://blog.rust-lang.org/2015/05/11/traits.html)
- [Rust Error Handling](https://blog.rust-lang.org/2016/09/09/Rust-1.12.html)

## 🚀 Running the Project

1. Navigate to the project directory:
```bash
cd rust-beginner-toolkit
```

2. Run the example:
```bash
cargo run
```

3. Follow the prompts to interact with the program.

## 🎉 Next Steps

- Explore Rust's package manager (crates.io)
- Learn about async/await with tokio
- Build a web API with actix-web
- Create a simple web application with yew (Rust + WebAssembly)

---

*This toolkit was created using AI-assisted learning techniques, demonstrating how modern AI tools can accelerate the learning process for new technologies.*
