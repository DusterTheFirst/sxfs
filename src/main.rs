#![feature(proc_macro_hygiene, decl_macro, type_alias_enum_variants)]
#![warn(clippy::all)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rust_embed;
#[macro_use] extern crate log;

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
mod logger;

use config::{Config, ConfigError};

lazy_static! {
    pub static ref HBS: Mutex<Handlebars> = Mutex::new(Handlebars::new());
}

fn load_config(config_path: &Path) -> Config {
    match Config::load(config_path) {
        Err(er) => {
            // Send error
            match er {
                ConfigError::Create(e) => error!(target: "config", "{} {:?}", "Error creating config file:".red(), e),
                ConfigError::Parse(e) => error!(target: "config", "{}\n{:#?}", "Error parsing config:".red(), e),
                ConfigError::Read(e) => error!(target: "config", "{} {:?}", "Error reading config file:".red(), e),
                ConfigError::Write(e) => error!(target: "config", "{} {:?}", "Error writing to config file:".red(), e),
            };
            // Panic
            panic!("Failed setting up config")
        },
        Ok(config) => {
            debug!(target: "config", "Loaded Config: {:#?}", config);
            config
        }
    }
}

fn main() -> std::io::Result<()> {
    // Init logger
    logger::init(log::LevelFilter::Trace).expect("Failed to initialize logger");
    
    error!("1");
    warn!("2");
    info!("3");
    debug!("4");
    trace!("5");

    trace!("{} {}", "Running SXFS from".green(), std::env::current_dir().unwrap().to_string_lossy().blue());

    // Load config
    debug!(target: "config", "{}", "Loading Config...".yellow());
    let config_path = Path::new("Config.toml");
    let config: Config = load_config(config_path);

    match fs::create_dir(&config.uploads_dir) {
        Ok(_) => info!("{} {}", "Created upload directory".yellow(), (&config.uploads_dir).to_string_lossy().blue()),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => debug!("{} {}", "Found upload directory".green(), (&config.uploads_dir).to_string_lossy().blue()),
            _ => return Err(e)
        }
    }

    // TODO: User defined templates
    // Regester templates
    HBS.lock().unwrap().set_strict_mode(true);
    debug!(target: "handlebars", "{}", "Loading templates...".yellow());
    for (template_file, error) in templates::load_templates() {
        match error {
            None => trace!("{} - {}", template_file.blue(), "OK".green()),
            Some(reason) => {
                error!("{} - {}", template_file.blue(), "FAIL".red());
                trace!("{}", reason);
            }
        }
    };
    debug!(target: "handlebars", "{}", "Loading partials...".yellow());
    for (partial_file, error) in templates::load_partials() {
        match error {
            None => trace!("{} - {}", partial_file.blue(), "OK".green()),
            Some(reason) => {
                error!("{} - {}", partial_file.blue(), "FAIL".red());
                trace!("{}", reason);
            }
        }
    };

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