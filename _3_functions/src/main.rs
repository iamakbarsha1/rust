// Functions
// entry point
// function / variable -> to be named in snake case
//! snake_case: (to go - recommended)
    // variable_name
// kebab-case: (not recommended)
    // variable-name

fn main() {
    hello_world();
    tell_heignt(40);
    // human_akbarsha(name: "Akbarsha", years_of_exp: 3.2, learn_lang: "Rust");
    human_akbarsha("Akbarsha", 3.2, "Rust");
}

// Hositing - can call funcitona anywhere in the code!
fn hello_world() {
    println!("Hello, world!");
}

// you can insert input values
fn tell_heignt(height: u32) {
    println!("Height: {} cm.", height);
}

// you can insert more than one input values
fn human_akbarsha(name: &str, years_of_exp: f32, learn_lang: &str) {
    println!("My name is {}, I have {} years of experience as a Software Engineer, currently learning {} lang!", name, years_of_exp, learn_lang);

    let _X: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty // or return price * qty
    };
    println!{"The price of _X is: {}", _X};

    let add_num: i32 = add(4, 5);
    println!("Add 2 nums: {}", add_num);


    let bmi: f64 = calculate_BMI(71.5, 1.8);
    println!("BMI: {:.2}", bmi);
}

// fn returing values:
    // -> arrow mark represents that the retun value should be i32 data type
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Constant:
// const _X = {
//     // code
// }
// const DEVELOPER_NAME: String = String::from("Akbar sha");
// const DEVELOPER_NAME: String = "Akbar sha".to_string();
// const DEVELOPER_NAME: &str = "Akbar sha";
// println!("Dev name: {}", DEVELOPER_NAME);


// Expression and Statements:
// Expression: 
    // Anything that returns a value
        // 5
        // true / false
        // add(3, 4)
        // if condition {val_1} else {val_2}
        // ({ code block })

// Statement: 
    // Anything that does not return a value
    // Almost all statement in RUST ends with semicolon (;)
    // let x = 10;
    
    // Example:
        // 1 Variable declaration: let x = 5;
        // 2 Function declaration: fn foo () {}
        // 3 Control flow statements: if condiiton { code } else { code }, while condition { code }



// Final Function Example:
fn calculate_BMI(weight: f64, height: f64) -> f64 {
    // formula: BMI = weight (kg) / height (m) ^ 2 
    weight / (height * height)
}