// Constants

fn main() {
    let x: i32 = 5;
    const Y: i32 = 70;
    // const NAME: String = String::from("Akbarsha");

    println!("Value of x: {}", x);
    println!("Value of Y: {}", Y);

    println!("Value of PI: {}", PI);

    println!("Value of 3 hours: {}", THREE_HOURS_IN_SECCONDS);
}

// You can declare a constant with a type annotation.
const PI: f64 = 3.141592653;
const THREE_HOURS_IN_SECCONDS: u32 = 60 * 60 * 3;