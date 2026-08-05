use rand::{RngExt, rng};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");

    let mut raphael: [i32; 11] = [0; 11];

    for item in 0..1_000_000 {
        let random_number = rng().random_range(0..=10);
        match random_number {
            0 => raphael[0] += 1,
            1 => raphael[1] += 1,
            2 => raphael[2] += 1,
            3 => raphael[3] += 1,
            4 => raphael[4] += 1,
            5 => raphael[5] += 1,
            6 => raphael[6] += 1,
            7 => raphael[7] += 1,
            8 => raphael[8] += 1,
            9 => raphael[9] += 1,
            10 => raphael[10] += 1,
            _ => (),
        } // match
    } // for

    for item in 0..=10 {
        println!("index[{:02}] = {}", item, raphael[item]);
    }

    println!("\n The End ...");
    Ok(())
}
