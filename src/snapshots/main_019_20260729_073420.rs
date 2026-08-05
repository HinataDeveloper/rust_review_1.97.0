//! Using checked method for number data type.

fn main() {
    println!("\n");

    let my_number: u128 = 2;

    let result_option = my_number.checked_pow(30_u32);

    let result = match result_option {
        Some(num) => num,
        None => panic!("->>> Error"),
    };

    println!("{}", result);

    println!("\nThe End ...");
}
