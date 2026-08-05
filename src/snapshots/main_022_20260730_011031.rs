use rand::RngExt;

fn main() {
    println!("\n");

    let rand_num: u8 = rand::rng().random_range(0..=255);
    let with:u32 = decimal_with(rand_num);
    let resultant = (rand_num as u128).checked_pow(with);
    println!("at last I found the way of using checked method. resultant: {}", resultant.unwrap());

    println!("\nThe End ...");
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum ConvertError {
    Overflow,
}

fn decimal_with(data: u8) -> u32 {
    match data {
        0..=9 => 1,
        10..=99 => 2,
        100..=255 => 3,
    }
}
