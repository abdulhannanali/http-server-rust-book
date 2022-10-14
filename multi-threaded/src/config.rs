use std::env;

const DEFAULT_PORT : i16 = 8000;

pub struct Config {
    pub port: i16,
    pub host: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            port: parse_port_from_env(),
            host: String::from("0.0.0.0")
        }       
    }
}


fn parse_port_from_env() -> i16 {
    let port_result = env::var("PORT");

    match port_result {
        Ok(p) => i16::from_str_radix(&p, 10).unwrap_or(DEFAULT_PORT),
        Err(_) => DEFAULT_PORT
    }
}
