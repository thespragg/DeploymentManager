use colour::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Default, Serialize, Deserialize)]
struct Config {
    location: String,
}

fn get_config_path() -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.push("config.toml");
    Ok(dir)
}

fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => return input,
        Err(_) => return "error".to_string(),
    }
}

fn create_config() -> io::Result<()> {
    let path = get_config_path()?;
    red_ln!("Config hasn't been set up yet, please enter the path to the centralised folder you want to use: ");
    let central_path = read_line();
    let mut cfg = File::create(path)?;
    let conf = Config {
        location: central_path,
    };

    cfg.write_all(toml::to_string(&conf).unwrap().as_bytes())
        .expect("err");

    Ok(())
}

pub fn change_location() {
    let path = get_config_path().unwrap();

    red_ln!("Enter the path you want to use as the centralised folder: ");
    let central_path = read_line();

    let conf = Config {
        location: central_path,
    };

    let mut file = File::open(path).unwrap();
    file.write_all(toml::to_string(&conf).unwrap().as_bytes())
        .expect("err");
        
    green_ln!("Path has been changed to: {}", conf.location)
}

pub fn load_config() -> std::io::Result<String> {
    let path = get_config_path()?;
    if !path.exists() {
        create_config()?
    }

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let cfg: Config = toml::from_str(&contents.to_string())?;
    Ok(cfg.location)
}
