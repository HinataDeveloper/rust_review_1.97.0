//! Convert u32 array data-type to a number.

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");

    let my_digit: [u128; 10] = [7, 9, 0, 5, 8, 6, 1, 3, 2, 4];
    let length = my_digit.len();
    let mut my_number = 0_u128;

    for (index, item) in (0..length).rev().enumerate() {
        my_number += my_digit[index] * (10 as u128).pow(item as u32);
    }

    println!("my_number is: {}", my_number);

    println!("\nThe End ...");
    Ok(())
}
