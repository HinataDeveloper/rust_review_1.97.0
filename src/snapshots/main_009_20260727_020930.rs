use std::error::Error;

use rand::RngExt;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");

    let mut index = 0;

    'abc: loop {
        let number = rand::rng().random_range(0..=10);

        match number {
            0 => println!("Zero"),
            1 => println!("One"),
            2 => println!("Two"),
            3 => println!("Three"),
            4 => println!("Four"),
            5 => println!("Five"),
            6 => println!("Six"),
            7 => println!("Seven"),
            8 => println!("Eight"),
            9 => println!("Nine"),
            10 => {
                println!("Ten");
                break 'abc;
            }
            _ => println!("Unknown number ..."),
        } // match
        index += 1;
    } // loop

    println!("index: {}", index + 1);

    println!("\nThe End ...\n");
    Ok(())
}
