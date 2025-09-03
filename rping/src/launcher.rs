use crate::controllers;

struct Config {
    action: String,
    // get action params
    dest: String,
    hostname: String,

    // serve action params
    port: u16
}

pub async fn launch_based_on_params(params: Vec<String>) -> Result<(), String> {
    let config: Config = parse_params(params)?;

    match config.action.as_str() {
        "serve" => {
            let _rocket = rocket::build()
                .mount("/", routes![controllers::index])
                .launch()
                .await.or_else(|_e| Err("Could not start Rocket server".to_string()))?;
            Ok(())
        },
        _ => Ok(())
    }

}

fn parse_params(params: Vec<String>) -> Result<Config, String> {
    let mut config = Config { action: String::new(), dest: String::new(), hostname: String::new(), port: 8000 };

    let mut i = 0;
    let len = params.len();
    while i < len {
        println!("{}", i);
        let mut param = params[i].to_owned();

        if param.starts_with("--") && i + 1 < len {
            config = add_param(config, param.split_off(2), &params[i + 1])
                .unwrap_or_else(|_e| panic!("Unknown option"));
            i = i + 1;
        }
        else {
            config = set_action(config, param).unwrap_or_else(|e| panic!("{e}"));
        }
        i = i + 1;
    }

    Ok(config)
}

fn add_param(mut config: Config, param: String, next_param: &str) -> Result<Config, &str> {
    match param.as_str() {
        "dest" => {
            config.dest = next_param.to_string();
            Ok(config)
        },
        "host" => {
            config.hostname = next_param.to_string();
            Ok(config)
        },
        "port" => {
            config.port = next_param.parse::<u16>().or_else(|_e| Err("Port is not integer"))?;
            Ok(config)
        },
        _ => Err("Unknown option")
    }
}

fn set_action(mut config: Config, param: String) -> Result<Config, String> {
    if !config.action.is_empty() {
        Err("Action set more than one time".to_string())
    }
    else {
        config.action = param;
        Ok(config)
    }
}
