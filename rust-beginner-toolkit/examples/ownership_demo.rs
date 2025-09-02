// This example demonstrates Rust's ownership system
// Run with: cargo run --example ownership_demo

fn main() {
    println!("Rust Ownership System Demo");
    println!("=========================");
    
    // Example 1: Basic ownership
    let s1 = String::from("hello");
    let s2 = s1; // s1's value is moved to s2
    // println!("{}", s1); // This would cause a compilation error!
    println!("s2: {}", s2);
    
    // Example 2: Borrowing (references)
    let s3 = String::from("world");
    let len = calculate_length(&s3); // We pass a reference
    println!("The length of '{}' is {}.", s3, len);
    
    // Example 3: Mutable borrowing
    let mut s4 = String::from("hello");
    change_string(&mut s4);
    println!("After change: {}", s4);
    
    // Example 4: Multiple immutable references
    let s5 = String::from("hello");
    let r1 = &s5;
    let r2 = &s5;
    println!("r1: {}, r2: {}", r1, r2);
    
    // Example 5: Demonstrating the borrowing rules
    let mut s6 = String::from("hello");
    {
        let r1 = &s6; // No problem
        let r2 = &s6; // No problem
        println!("r1: {}, r2: {}", r1, r2);
    } // r1 and r2 go out of scope here
    
    let r3 = &mut s6; // No problem
    println!("r3: {}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(" world");
}
