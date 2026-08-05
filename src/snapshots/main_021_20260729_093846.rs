fn main() {
    println!("\n");

    let raphael: i128 = 120;
    let result = raphael.checked_pow(7).ok_or(MyError::Overflow);

    match result {
        Ok(num) => println!("number is: {}", num),
        Err(err) => panic!("Error: {:?}", err),
    }

    println!("\nThe End ...");
}

#[derive(Debug)]
enum MyError {
    Overflow,
}
