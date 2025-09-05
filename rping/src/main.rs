#[macro_use]
extern crate rocket;
use std::env;

use launcher::launch_based_on_params;

pub mod launcher;
pub mod routes;
pub mod types;

#[rocket::main]
async fn main() -> Result<(), &'static str> {
    let mut args: Vec<String> = env::args().collect();
    launch_based_on_params(args.split_off(1)).await
}
