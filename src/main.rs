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
use sxfs::templates::{
    uploader::{ShortenerTemplate, UploaderTemplate},
    UpdatableTemplate,
};
use sxfs::routes;

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
            routes::internal_error,
            routes::not_found,
            routes::unauthorized,
        ])
        .mount(
            "/",
            routes![
                routes::index,
                routes::login_form,
                routes::login_submit,
                routes::logout,
                routes::public_files,
                routes::redirect_short_link,
                routes::redirect_to_upload,
                routes::shorten,
                routes::upload,
                routes::uploaders,
                routes::view_upload,
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
