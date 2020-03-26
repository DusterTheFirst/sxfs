//! Tools to assist the ShareX File Server

#![feature(proc_macro_hygiene, decl_macro)]
#![warn(missing_docs)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;

pub mod args;
pub mod auth;
pub mod config;
pub mod generate;
pub mod id;
pub mod routes;
pub mod templates;
pub mod user;
