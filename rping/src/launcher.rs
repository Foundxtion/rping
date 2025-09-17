use rocket::futures::lock::Mutex;
use rocket_krb5::{KrbFairing, KrbServerCreds};
use std::collections::HashMap;

use crate::{routes, types::HostMap};

/// Configuration struct for application launch parameters.
/// Used to store options for serving, listing, or sending DNS records.
struct Config {
    // common options [serve, list, send]
    action: String,
    url: String,

    // serve action params
    port: u16,
    principal: String,

    // list send action params
    realm: String,
}

/// Launches the application based on parsed command-line parameters.
///
/// ### Parameters
/// - `params`: Vector of command-line arguments.
///
/// ### Returns
/// - `Result<(), &'static str>`: Ok if launch is successful, Err otherwise.
///
/// ### Example
/// ```rust
/// let params = vec!["serve".to_string(), "--url".to_string(), "http://localhost:8000".to_string()];
/// launch_based_on_params(params).await.unwrap();
/// ```
pub async fn launch_based_on_params(params: Vec<String>) -> Result<(), &'static str> {
    let config: Config = parse_params(params)?;

    match config.action.as_str() {
        "serve" => {
            let auth_fairing = KrbFairing {};
            let creds: KrbServerCreds = KrbServerCreds::new(config.principal)
                .ok_or_else(|| "Cannot instantiate kerberos creds")?;
            println!("{}", creds.principal.clone());

            let _rocket = rocket::build()
                .mount("/add", routes![routes::post_address])
                .mount("/get", routes![routes::get_list])
                .manage(HostMap::new(HashMap::new()))
                .manage(Mutex::new(creds))
                .attach(auth_fairing)
                .launch()
                .await
                .or_else(|_e| Err("Could not start Rocket server"))?;
            Ok(())
        }
        "list" => {
            librping::list(config.url, config.realm).await;
            Ok(())
        }
        "send" => {
            librping::send(config.url, config.realm).await;
            Ok(())
        }
        _ => Err("Unknown command"),
    }
}

/// Parses command-line parameters into a Config struct.
///
/// ### Parameters
/// - `params`: Vector of command-line arguments.
///
/// ### Returns
/// - `Result<Config, &'static str>`: Ok with Config struct, Err otherwise.
fn parse_params(params: Vec<String>) -> Result<Config, &'static str> {
    let mut config = Config {
        action: String::new(),
        url: String::new(),
        port: 8000,
        principal: String::new(),
        realm: String::new(),
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

    config = config_valid(config)?;
    Ok(config)
}

/// Adds a parameter to the Config struct.
///
/// ### Parameters
/// - `config`: The current Config struct.
/// - `param`: The parameter name as a string.
/// - `next_param`: The value for the parameter.
///
/// ### Returns
/// - `Result<Config, &str>`: Ok with updated Config, Err otherwise.
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
        "principal" => {
            config.principal = next_param.to_string();
            Ok(config)
        }
        "realm" => {
            config.realm = next_param.to_string().to_uppercase();
            Ok(config)
        }
        _ => Err("Unknown option"),
    }
}

/// Sets the action field in the Config struct.
///
/// ### Parameters
/// - `config`: The current Config struct.
/// - `param`: The action name as a string.
///
/// ### Returns
/// - `Result<Config, &'static str>`: Ok with updated Config, Err otherwise.
fn set_action(mut config: Config, param: String) -> Result<Config, &'static str> {
    if !config.action.is_empty() {
        Err("Action set more than one time")
    } else {
        config.action = param;
        Ok(config)
    }
}

/// Validates the Config struct for required fields.
///
/// ### Parameters
/// - `config`: The Config struct to validate.
///
/// ### Returns
/// - `Result<Config, &'static str>`: Ok if valid, Err otherwise.
fn config_valid(config: Config) -> Result<Config, &'static str> {
    if config.action.is_empty() {
        return Err("No action provided");
    }

    if !config.action.contains("serve") && config.url.is_empty() {
        return Err("No url specified");
    }

    if !config.action.contains("serve") && config.realm.is_empty() {
        return Err("No realm specified");
    }

    if config.action.contains("serve") && config.principal.is_empty() {
        return Err("No kerberos principal specified");
    }

    Ok(config)
}
