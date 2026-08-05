//! Create a number with five digit.
//! index 0 must not be zero.
//! any of five digit must not be repeated in the number.

use tracing::info;
use tracing_subscriber::EnvFilter;

use rand::RngExt;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");

    init_logging();
    info!("Program Started ...");

    for _ in 0..10 {
        let mut my_number: [u32; 5] = [0; 5];

        let mut num = rand::rng().random_range(1..=9);
        my_number[0] = num;

        for index in 1..=4 {
            while my_number.contains(&num) {
                num = rand::rng().random_range(0..=9);
            }
            my_number[index] = num;
        }

        for (index, _) in (0..5).enumerate() {
            print!("{}", my_number[index]);
        }
        println!();
    }

    println!("\nThe End ...");
    Ok(())
}

fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .init();
}
