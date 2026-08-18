// Date: Thu Aug 19 2026

// Project: Review The Book Rust Programming Language
// Goal: How to use vector. Ownership problem with using Vector
// Dependency: Without dependency

// rustc 1.100.0-nightly (34baba539 2026-08-16)
// binary: rustc
// commit-hash: 34baba5394fcbda4cba7b7c1964a6db421c77c91
// commit-date: 2026-08-16
// host: x86_64-unknown-linux-gnu
// release: 1.100.0-nightly
// LLVM version: 23.1.0

// cargo 1.100.0-nightly (8a0d8afba 2026-08-15)
// release: 1.100.0-nightly
// commit-hash: 8a0d8afba810304bcf9a10bac430be80dd470233
// commit-date: 2026-08-15
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Ubuntu 26.4.0 (resolute) [64-bit]
// Kernel Version: 7.0.14-070014-generic
// Firmware Version: 71CN51WW(V1.21)

// rustup 1.29.0 (28d1352db 2026-03-05)

fn main() {
    println!("\n");

    let mut number_vector: Vec<u32> = Vec::new();

    let my_range = 0..10000;

    for item in my_range {
        number_vector.push(item);
    }

    for item in number_vector.iter() {
        println!("->> {}", item);
    }

    let index_800 = &number_vector[800];

    // encounter with compile time error. because in up line
    // number vector was borrowed as immutable and can not has 
    // borrowed as mutable. This is duo to below line is consuming 
    // index_800 variable.
    number_vector.push(1200);

    println!("value of index 800 is: {}", index_800);

    println!("\nThe End ...\n");
}
