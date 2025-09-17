#[macro_use]
extern crate rocket;
use std::env;

use launcher::launch_based_on_params;

pub mod launcher;
pub mod routes;
pub mod types;

/// Main entry point for the application.
/// Parses command-line arguments and launches the application.
///
/// ### Returns
/// - `Result<(), &'static str>`: Ok if successful, Err otherwise.
#[rocket::main]
async fn main() -> Result<(), &'static str> {
    let mut args: Vec<String> = env::args().collect();
    launch_based_on_params(args.split_off(1)).await
}
