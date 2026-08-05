//! Create a five-digit number.
//! The first digit must not be zero.
//! No digit may be repeated.

use rand::seq::SliceRandom;
use std::error::Error;
use tracing::info;
use tracing_subscriber::EnvFilter;

fn main() -> Result<(), Box<dyn Error>> {
    init_logging();
    info!("Program started");

    for _ in 0..10 {
        let number = generate_five_digit_number();

        for digit in number {
            print!("{digit}");
        }
        println!();
    }

    info!("\nThe End ...");
    Ok(())
}

fn generate_five_digit_number() -> [u8; 5] {
    let mut digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut rng = rand::rng();

    digits.shuffle(&mut rng);

    // Ensure the first digit is not zero.
    if digits[0] == 0 {
        digits.swap(0, 1);
    }


    [digits[0], digits[1], digits[2], digits[3], digits[4]]
}

fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .init();
}
