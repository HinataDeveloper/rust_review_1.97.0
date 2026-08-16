// Date: Thu Aug 15 2026

// Project: Review Rust version 1.97.0
// Goal: Using Structure
// Dependency: Without dependency

// Hardware Mode: Lenovo IdeaPad Z500
// Processor: Intel® Core™ i7-3612QM × 8

// OS: Ubuntu 26.04 LTS x86_64 GNU/Linux
// Kernel Version: 7.0.14-070014-generic
// Firmware Version: 71CN51WW(V1.21)

// Graphics: Intel® HD Graphics 4000 (IVB GT2)
// GNOME Version: 50
// Windowing System: Wayland

// rustc 1.99.0-nightly (d453bdd8f 2026-08-14)
// cargo 1.99.0-nightly (eb98b54bc 2026-08-11)
// rustup 1.29.0 (28d1352db 2026-03-05)

fn main() {
    println!("\n");

    let raphael: User = User {
        active: true,
        username: String::from("@raphael"),
        email: String::from("raphael@gmail.com"),
        signing_count: 1,
    };

    println!("value of raphael is: {:?}", raphael);

    let raphel2: User = User {
        email: String::from("raphael2@gmail.com"),
        ..raphael
    };

    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("value of raphael2: {:?}", raphel2);

    println!("\nThe End ...");
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    signing_count: u32,
}
