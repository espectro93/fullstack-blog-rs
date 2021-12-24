use dotenv::dotenv;
use std::env;

fn main() -> Result<(), String> {
    dotenv().ok();

    let config_name = env::var("CONFIG_ENV").expect("CONFIG must be set");
    let rocket = backend::rocket_factory(&config_name)?;
    rocket.launch();
    Ok(())
}