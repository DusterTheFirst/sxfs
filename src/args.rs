//! The arguments that are passed to the program on the command line

use std::path::PathBuf;
use structopt::StructOpt;

/// The command line arguments passed to sxfs
#[derive(StructOpt, Debug)]
#[structopt(name = "sxfs")]
pub struct Args {
    /// Enable informational logging
    #[structopt(short, long)]
    pub info: bool,
    /// Enable debug logging (requires info logging)
    #[structopt(short, long, requires = "info")]
    pub debug: bool,
    /// Enable trace logging (requires debug logging)
    #[structopt(short, long, requires = "debug")]
    pub trace: bool,
    /// The path to the config file
    #[structopt(short, long, default_value = "data/config.toml")]
    pub config: PathBuf,
    /// The path to output the ShareX custom uploader file
    #[structopt(short, long, default_value = "data/uploaders/uploader.sxcu")]
    pub uploader: PathBuf,
    /// The path to output the ShareX custom URL shortener file
    #[structopt(short, long, default_value = "data/uploaders/shortener.sxcu")]
    pub shortener: PathBuf,
}
