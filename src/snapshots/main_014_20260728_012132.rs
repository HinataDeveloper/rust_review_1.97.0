//! Creating five-digit number.
//! First number must not be zero.
//! No digit may not be repeat.

use std::error::Error;

use rand::seq::SliceRandom;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");

    for _ in 0..10 {
        let resultant = get_five_digit_number();
        show_array(&resultant);
    }

    println!("\nThe End ...");
    Ok(())
}

fn get_five_digit_number() -> [u32; 5] {
    let mut digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut rng = rand::rng();
    digits.shuffle(&mut rng);

    if digits[0] == 0 {
        digits.swap(0, 1);
    }

    [digits[0], digits[1], digits[2], digits[3], digits[4]]
}

fn show_array(arr: &[u32; 5]) {
    for item in arr.iter() {
        print!("{}", item);
    }
    println!();
}
