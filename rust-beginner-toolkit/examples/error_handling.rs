// This example demonstrates Rust's error handling
// Run with: cargo run --example error_handling

use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("Rust Error Handling Demo");
    println!("========================");
    
    // Example 1: Option type
    let numbers = vec![1, 2, 3, 4, 5];
    
    match find_number(&numbers, 3) {
        Some(index) => println!("Found 3 at index: {}", index),
        None => println!("3 not found in the vector"),
    }
    
    // Example 2: Result type with file operations
    match read_file_content("nonexistent.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
    
    // Example 3: Using ? operator (simulated)
    if let Err(e) = process_data() {
        println!("Processing error: {}", e);
    }
    
    // Example 4: Custom error types
    match divide_numbers(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Division error: {}", e),
    }
    
    match divide_numbers(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Division error: {}", e),
    }
}

// Function that returns Option
fn find_number(numbers: &[i32], target: i32) -> Option<usize> {
    for (index, &number) in numbers.iter().enumerate() {
        if number == target {
            return Some(index);
        }
    }
    None
}

// Function that returns Result
fn read_file_content(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// Function that uses the ? operator
fn process_data() -> Result<(), String> {
    // Simulate some processing that might fail
    let success = true; // Change to false to see error handling
    
    if success {
        Ok(())
    } else {
        Err("Processing failed".to_string())
    }
}

// Custom error handling for division
fn divide_numbers(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero is not allowed".to_string())
    } else {
        Ok(a / b)
    }
}
