fn main() {
    println!("Compound data types!");

// Compound data types
    // arrays, tuples, slices, strings (slice string)

// Arrays
    let nums: [i32; 5] = [0, 1, 2, 3, 4];
    println!("Number arr: {:?}", nums);

    let fruits: [&str; 3] = ["apple", "banana", "carrot"];
    println!("Friiuts arr: {:?}", fruits);
    println!("Fruit 1: {}  ", fruits[0]);

// Tuples
    // let human: (&str, i32, bool) = ("Akbarsha", 22, true);
    let human: (String, i32, bool) = ("Akbarsha".to_string(), 22, true);
    println!("Tuple: Human: {:?}", human);

    let my_mix_tuple = ("Hamza", 3, true, nums);
    println!("My mix tuple: {:?}", my_mix_tuple);

// Slices: [1,2,3,4,5] - contigeous sequence of elements - continous
    let number_slices: &[i32] = &[1,2,3,4,5];
    println!("Slices: Number: {:?} ", number_slices);

    let name_slices: &[&str] = &["Akbarsha", "Hamza", "Mohammed"];
    println!("Slices: Names: {:?} ", name_slices);

    let place_slices: &[String] = &["Chennai".to_string(), "Makkah".to_string(), "Madinah".to_string()];
    println!("Slices: PLace: {:?}", place_slices);

// String vs String Slices (&str)
    // String: 
        // - growable/expandable/inscreasable/decreasable
        // - mutable - push, delete items
        // - owned string types
    
    let mut my_string: String = String::from("My name is ");
    my_string.push_str("Akbar SHa!");
    println!("My string: {}", my_string);

    // String SLices:
        // B- &str (String Slice)
        // 
    let string: String = String::from("Assalamu alaikum wrwb, Akbarsha!");
    let slice: &str = &string;
    let slice_4: &str = &string[0..21];
    println!("Slice value: {}", slice);
    println!("Slice value till index 4: {}", slice_4);
    







}
