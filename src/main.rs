//! ShareX File Server
//!
#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::helmet::SpaceHelmet;
use simplelog::{
    CombinedLogger, ConfigBuilder as LogConfigBuilder, LevelFilter, SharedLogger, SimpleLogger,
    TermLogger, TerminalMode,
};
use structopt::StructOpt;

use colored::*;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;

use sxfs::args::Args;
use sxfs::config::Config;
use sxfs::routes;
use sxfs::templates::{
    uploader::{ShortenerTemplate, UploaderTemplate},
    UpdatableTemplate,
};

fn main() -> std::io::Result<()> {
    // Load args first
    let args: Args = Args::from_args();

    // Init logger
    CombinedLogger::init(vec![
        create_logger(
            "sxfs",
            if args.trace {
                LevelFilter::Trace
            } else if args.debug {
                LevelFilter::Debug
            } else if args.info {
                LevelFilter::Info
            } else {
                LevelFilter::Warn
            },
        ),
        create_logger(
            "rocket",
            if args.info {
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
    match UploaderTemplate::new(&config).update(&args.uploader) {
        Ok(()) => {}
        Err(e) => {
            error!("{} {}", "Failed to write uploader template:".red(), e);
            panic!("{:?}", e);
        }
    }
    match ShortenerTemplate::new(&config).update(&args.shortener) {
        Ok(()) => {}
        Err(e) => {
            error!("{} {}", "Failed to write shortener template:".red(), e);
            panic!("{:?}", e);
        }
    }

    // Start web interface
    rocket::ignite()
        .register(catchers![
            routes::catcher::internal_error,
            routes::catcher::not_found,
            routes::catcher::unauthorized,
        ])
        .mount(
            "/",
            routes![
                routes::index,
                routes::auth::login_form,
                routes::auth::login_submit,
                routes::auth::logout,
                routes::public_files,
                routes::uploaders,
            ],
        )
        .mount(
            "/u",
            routes![
                routes::upload::view,
                routes::upload::all,
                routes::upload::create,
                routes::upload::delete,
            ],
        )
        .mount(
            "/;",
            routes![
                routes::link::follow,
                routes::link::all,
                routes::link::create,
                routes::link::delete,
            ],
        )
        .manage(config)
        .attach(SpaceHelmet::default())
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
