fn main() {

// Rust has signed (- and +) and unsigned (only +) integer

// Integers types

    // Types of diff sizes:
        // - i8, i16, i32, i64, i128: Signed integers
        // - u8, u16, u32, u64, u128: Unsigned integers

    println!("Data types!");

    let x = 5;
    println!{"The value of x is {}", x}
    
    let x = "Akbarsha";
    println!{"The value of x is {x}"}

    let y: i8 = 127;
    let z: u8 = 255;
    println!{"The value of y is {}", y}
    println!{"The value of z is {}", z}

// Floats [Floating point integers]

    // f32, f64

    let pi: f64 = 3.14;
    println!{"Value of pi is {}", pi}

// Boolean: true, false

    let is_raining: bool = true;
    println!{"is_raining: {}", is_raining}

// Character type

    // char

    // let name: char = 'a';
    let name: char = 'a';
    println!("My name is: {}", name)
}

