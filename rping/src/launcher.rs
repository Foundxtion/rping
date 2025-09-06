use std::collections::HashMap;

use crate::{routes, types::HostMap};

struct Config {
    // common options [serve, list, send]
    action: String,
    url: String,

    // serve action params
    port: u16,
}

pub async fn launch_based_on_params(params: Vec<String>) -> Result<(), &'static str> {
    let config: Config = parse_params(params)?;

    match config.action.as_str() {
        "serve" => {
            let _rocket = rocket::build()
                .mount("/add", routes![routes::post_address])
                .mount("/get", routes![routes::get_list])
                .manage(HostMap::new(HashMap::new()))
                .launch()
                .await
                .or_else(|_e| Err("Could not start Rocket server"))?;
            Ok(())
        }
        "list" => {
            librping::list(config.url).await;
            Ok(())
        },
        "send" => {
            librping::send(config.url).await;
            Ok(())
        },
        _ => Err("Unknown command"),
    }
}

fn parse_params(params: Vec<String>) -> Result<Config, &'static str> {
    let mut config = Config {
        action: String::new(),
        url: String::new(),
        port: 8000,
    };

    let mut i = 0;
    let len = params.len();
    while i < len {
        let mut param = params[i].to_owned();

        if param.starts_with("--") && i + 1 < len {
            config = add_param(config, param.split_off(2), &params[i + 1])
                .or_else(|_e| Err("Unknown option"))?;
            i = i + 1;
        } else {
            config = set_action(config, param)?;
        }
        i = i + 1;
    }

    Ok(config)
}

fn add_param(mut config: Config, param: String, next_param: &str) -> Result<Config, &str> {
    match param.as_str() {
        "url" => {
            config.url = next_param.to_string();
            Ok(config)
        }
        "port" => {
            config.port = next_param
                .parse::<u16>()
                .or_else(|_e| Err("Port is not integer"))?;
            Ok(config)
        }
        _ => Err("Unknown option"),
    }
}

fn set_action(mut config: Config, param: String) -> Result<Config, &'static str> {
    if !config.action.is_empty() {
        Err("Action set more than one time")
    } else {
        config.action = param;
        Ok(config)
    }
}
