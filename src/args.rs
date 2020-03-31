//! The arguments that are passed to the program on the command line

use std::{net::IpAddr, path::PathBuf};
use structopt::StructOpt;

/// A file server for handling uploads from the ShareX client
#[derive(StructOpt, Debug)]
#[structopt(name = "sxfs")]
pub struct Args {
    /// Enable verbose logging. (1 = informational, 2 = debug, 3 = trace)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,
    /// Enable rocket info logging (requires info logging)
    #[structopt(short, long, requires = "verbose")]
    pub rocket_log: bool,

    /// The port to bind to
    #[structopt(short, long, default_value = "8000")]
    pub port: u16,
    /// The address to bind to
    #[structopt(short, long, default_value = "0.0.0.0")]
    pub address: IpAddr,

    /// The path to the config file
    #[structopt(short, long, default_value = "data/config.toml")]
    pub config: PathBuf,
    /// The path to output the generated ShareX custom uploaders file
    #[structopt(short, long, default_value = "data/uploaders")]
    pub uploaders: PathBuf,
    /// The path to the sqlite database that holds the mappings between uploads and their files aswell as
    #[structopt(short = "db", long, default_value = "data/db.sqlite")]
    pub database: PathBuf,
}
