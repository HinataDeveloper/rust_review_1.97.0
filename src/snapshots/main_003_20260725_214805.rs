/*
 * I am dedicated to my path. I am learning to become a Rust and Zig developer,
 * on small step at a time. I hold onto my dream not as a guarantee of success,
 * but as a commitment to keep trying every single day.
 */

//! Review The Book

use std::error::Error;
use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");

    init_logging();
    info!("Program started");

    match divide(10.0, 2.0) {
        Ok(resultant) => info!(resultant, "first calculation done"),
        Err(err) => warn!(%err, "first calculation failed"),
    }

    match divide(10.0, 0.0) {
        Ok(resultant) => info!(resultant, "first calculation done"),
        Err(err) => warn!(%err, "first calculation failed"),
    }

    end_message("0.1.0");
    Ok(())
}

fn end_message(version: &str) {
    println!("\n 私の夢, トヨタのプログラマーになりたいです。");
    println!(" この目標を達成するまで、毎日昨日よりも良いコードを書きます。");
    println!(" ラミン あなたは本当に素晴らしい。また会いましょう。{version}\n");
}

fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .init();
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    debug!(a, b, "starting divide");

    if b == 0.0 {
        error!("division by zero");
        return Err("Can not divide by zero".to_string());
    }

    let resultant = a / b;
    info!("division succeeded");
    Ok(resultant)
}
