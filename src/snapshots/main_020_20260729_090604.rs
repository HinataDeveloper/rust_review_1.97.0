fn main() {
    println!("\n");

    let my_array = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    let zero = "0".to_string();

    let sum = my_array.iter().fold(zero, |acc, x| format!("({acc} + {x})"));

    println!("sum is: {}", sum);

    println!("\nThe End ...");
}
