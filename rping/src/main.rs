use std::env;

#[macro_use] extern crate rocket;
pub mod controllers;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let args: Vec<String> = env::args().collect();

    if args.iter().count() > 1 {
        println!("{}", args[1]);
    }

    let _rocket = rocket::build()
        .mount("/", routes![controllers::index])
        .launch()
        .await?;
    Ok(())
}
