use rng::gen_random;
use std::{
    env,
    io::{self},
};

/// Produces an array of random integers.
/// Generates an array of random integers. Returns a vector if successful. Writes
/// errors to the provided writer.
///
/// Correct usage:
/// `cargo run 20 -5 5`
/// The command above requests 20 random integers between -5 and 5.
fn main() {
    let request: Vec<String> = env::args().collect();
    if let Some(output) = gen_random(&request, &mut io::stderr()) {
        println!("\n{:?}", output);
    };
}
