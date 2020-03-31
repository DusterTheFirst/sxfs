//! ShareX File Server
#![feature(proc_macro_hygiene, decl_macro)]

use colored::*;
use io::{Error, ErrorKind};
use rocket::{
    config::{Environment, Value},
    fairing::AdHoc,
    http::Header,
};
use rocket_contrib::helmet::SpaceHelmet;
use simplelog::{
    CombinedLogger, ConfigBuilder as LogConfigBuilder, LevelFilter, SharedLogger, SimpleLogger,
    TermLogger, TerminalMode,
};
use std::{collections::HashMap, io};
use structopt::StructOpt;
use sxfs::args::Args;
use sxfs::config::Config;
use sxfs::routes;
use sxfs::{
    create_parent_directories,
    database::Database,
    templates::{
        uploader::{ShortenerTemplate, UploaderTemplate},
        UpdatableTemplate,
    },
};

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;

fn main() -> io::Result<()> {
    // Load args first
    let args: Args = Args::from_args();

    // Init logger
    CombinedLogger::init(vec![
        create_logger(
            "sxfs",
            match args.verbose {
                0 => LevelFilter::Warn,
                1 => LevelFilter::Info,
                2 => LevelFilter::Debug,
                3 => LevelFilter::Trace,
                _ => LevelFilter::Off,
            },
        ),
        create_logger(
            "rocket",
            if args.rocket_log {
                LevelFilter::Info
            } else {
                LevelFilter::Warn
            },
        ),
    ])
    .ok();

    // Load config
    debug!("{}", "Loading Config...".yellow());
    let config: Config = match Config::load(&args.config) {
        Err(er) => {
            // Send error
            error!("{} {}", "Failed to process config file:".red(), er);
            // Panic
            panic!("{:?}", er);
        }
        Ok(config) => {
            debug!("Loaded Config: {:#?}", config);
            config
        }
    };

    // Write out uploaders
    match UploaderTemplate::new(&config).update(&args.uploaders.join("uploader.sxcu")) {
        Ok(()) => {}
        Err(e) => {
            error!("{} {}", "Failed to write uploader template:".red(), e);
            panic!("{:?}", e);
        }
    }
    match ShortenerTemplate::new(&config).update(&args.uploaders.join("shortener.sxcu")) {
        Ok(()) => {}
        Err(e) => {
            error!("{} {}", "Failed to write shortener template:".red(), e);
            panic!("{:?}", e);
        }
    }

    // Create parent directories for database
    create_parent_directories(&args.database)?;

    // Configure contrib database for rocket
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    database_config.insert("url", Value::from(args.database.to_string_lossy().as_ref()));
    databases.insert("db", Value::from(database_config));

    // Configure web interface
    let rocket_config = rocket::Config::build(
        Environment::active().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?,
    )
    .address(args.address.to_string())
    .port(args.port)
    .extra("databases", databases)
    .finalize()
    .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    // Start web interface
    rocket::custom(rocket_config)
        .register(catchers![
            routes::catcher::internal_error,
            routes::catcher::not_found,
            routes::catcher::unauthorized,
        ])
        .mount(
            "/",
            routes![
                routes::auth::login_form,
                routes::auth::login_submit,
                routes::auth::logout,
                routes::index,
                routes::link::all,
                routes::link::create,
                routes::link::delete,
                routes::link::follow,
                routes::public_files,
                routes::upload::all,
                routes::upload::create,
                routes::upload::delete_by_id,
                routes::upload::delete,
                routes::upload::view_by_id,
                routes::upload::view,
                routes::uploaders,
            ],
        )
        .manage(config)
        .attach(SpaceHelmet::default())
        .attach(AdHoc::on_response("No-Cache", |_, res| {
            res.set_header(Header::new(
                "Cache-Control",
                "no-store, no-cache, must-revalidate, max-age=0",
            ));
        }))
        .attach(Database::fairing())
        .launch();

    Ok(())
}

/// Create a configured logger with the specified settings
fn create_logger(filter: &'static str, level: LevelFilter) -> Box<dyn SharedLogger> {
    let config = LogConfigBuilder::new().add_filter_allow_str(filter).build();

    match TermLogger::new(level, config.clone(), TerminalMode::Mixed) {
        None => SimpleLogger::new(level, config),
        Some(log) => log,
    }
}
