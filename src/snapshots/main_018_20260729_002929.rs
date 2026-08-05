//! My Tiny Rust Programming Language Code
//! Date: Tue Jul 28 2026
//! Goal: Convert an array number data-type to a number.
//! Dependency: without dependency
//! Rust Version: nightly 1.99.0
//! For: Analyze, review and writing idiomatic version

fn main() {
    println!("\n");

    let int_array: [u8; 5] = [1, 2, 3, 4, 5];
    let result = convert_array_to_number(&int_array);
    println!("array: {:?} convert to: {}", int_array, result);

    let int_array: [u8; 5] = [10, 20, 130, 40, 255];
    let result = convert_array_to_number(&int_array);
    println!("array: {:?} convert to: {}", int_array, result);

    println!("\nThe End ...");
}

fn convert_array_to_number(data: &[u8]) -> u128 {
    let mut str_num: String = String::new();

    for item in data.iter() {
        str_num.push_str(&item.to_string());
    }

    str_num.trim().parse().unwrap_or_default()
}
