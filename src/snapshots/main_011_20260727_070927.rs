use rand::{RngExt, rng};
use std::{collections::BTreeMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");

    let mut raphael: [i32; 11] = [0; 11];

    for _ in 0..1_000_000 {
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

    let mut sort: BTreeMap<i32, i32> = BTreeMap::new();

    for (index, _) in (0..=10).enumerate() {
        sort.insert(raphael[index], index as i32);
        //println!("index[{:02}] = {}", item, raphael[item]);
    }

    for value in sort.iter() {
        println!("{} -> {}", value.0, value.1);
    }

    println!("\n The End ...");
    Ok(())
}
