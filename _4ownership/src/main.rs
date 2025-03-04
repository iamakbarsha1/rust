// Ownership, Borrowing and References

// -----------------------
// Ownership:
    // C, C++ -> Memory managament control issue
    // Garbage collector -> resolved the abv issue, but created a new issue!
    // New issue with garbage collector: to clean up the memory/data, it will stop/freeze the program
    //! New Issue resulting in Slow Performance: [stopping/Resuming the program]

// Ownership:
    // [stopping/Resuming the program]
    // OWNERSHIP: 
        // introduced by Rust to solve memory safety issues and high performance at the same time.
    
    // What is ownership?
        // Every value has a single owner [every variable has one value, and it is its sole owner]
    // Borrowing & References: 
        // Borrowing in a nut shell allows you to temporarily borrow references to values
        // this acrually enables SAFE CONCURRENT access without sacrificing the memory safety

// Ownership rules:
    // 1. Each value in Rust has an OWNER (a variable that's its owner.)
    // 2. There can be ONLY ONE OWNER at a time
    // 3. When the owner goes out of scope, the value will be dropped.


//! 1 - EXAMPLE: Each value in Rust has an OWNER (a variable that's its owner.) || Each value in Rust has a variable that's its owner.

// fn main() {
//     let s1: String = String::from("RUST");
//     let len = calculate_length(&s1);
//     println!("Length of {} is {}", s1, len);
// }

// fn calculate_length (s: &String) -> usize {
//     return s.len();
//     // s.len()
// }

// ! 2 - EXAMPLE: There can be ONLY ONE OWNER at a time

// fn main () {
//     let s1 = String::from("RUST");
//     let s2 = s1;
//     // println!("{}", s1); // throws err
//     println!("{}", s2);
// }

//! 3 - EXAMPLE - When the owner goes out of scope, the value will be dropped.

fn main () {
    let s1 = String::from("RUST");
    let _len = calculate_length(&s1);
} //? s1 goes out of scope and its value will be dropped

fn printLost(s: &string) {
    println!("{}", &s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}