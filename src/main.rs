#![feature(proc_macro_hygiene, decl_macro, type_alias_enum_variants)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rust_embed;

use std::path::{Path};
use std::io::ErrorKind;
use std::sync::Mutex;
use std::fs;

use handlebars::Handlebars;

use colored::*;

mod gaurds;
mod paths;
mod templates;
mod config;

use config::Config;

lazy_static! {
    pub static ref HBS: Mutex<Handlebars> = Mutex::new(Handlebars::new());
}

fn main() -> std::io::Result<()> {
    // TODO: Config
    println!("{} {}", "Running SXFS from".green(), format!("{:?}", std::env::current_dir().unwrap()).blue());

    let uploads_dir = Path::new("uploads");
    match fs::create_dir(uploads_dir) {
        Ok(()) => println!("{} {}", "Created upload directory".yellow(), format!("{:?}", uploads_dir).blue()),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => println!("{} {}", "Found upload directory".green(), format!("{:?}", uploads_dir).blue()),
            _ => return Err(e)
        }
    }

    // TODO: User defined templates
    // Regester templates
    HBS.lock().unwrap().set_strict_mode(true);
    println!("{}", "Loading templates...".yellow());
    templates::load_templates().unwrap();
    println!("{}", "Loading partials...".yellow());
    templates::load_partials().unwrap();

    // Load config
    println!("{}", "Loading Config...".yellow());
    let config_path = Path::new("Config.toml");
    let config_file = fs::read_to_string(config_path);
    let config_file = match config_file {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                let config_default = toml::to_string(&Config::default()).unwrap();
                
                fs::write(config_path, &config_default).unwrap();

                config_default
            },
            _ => {
                println!("{:?}", e);
                return Ok(());
            }
        },
        Ok(file) => file
    };
    let config: Config = match toml::from_str(&config_file) {
        Ok(c) => c,
        Err(e) => {
            println!("{}\n{:#?}", "Error parsing config".red(), e);
            return Ok(());
        }
    };

    println!("{:#?}", config);

    rocket::ignite().register(catchers![
        paths::not_found,
        paths::unauthorized
    ]).mount("/", routes![
        paths::index,
        paths::view_upload,
        paths::redirect_short_url,
        paths::make_upload
    ]).launch();

    Ok(())
}