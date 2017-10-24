extern crate rustdoc_static;

#[macro_use]
extern crate error_chain;

extern crate clap;
extern crate handlebars;
extern crate jsonapi;
extern crate pretty_env_logger;

use std::io::prelude::*;
use std::io;

use clap::{Arg, App};
use jsonapi::api::JsonApiDocument;

use rustdoc_static::errors::*;

fn run() -> Result<()> {
    pretty_env_logger::init().unwrap();

    let matches = App::new("rustdoc-frontend")
        .arg(
            Arg::with_name("output")
                .long("output")
                .short("o")
                .takes_value(true)
                .help("where the documentation should be output")
                .required(true),
        )
        .get_matches();

    let output_path = matches.value_of("output").unwrap();

    let mut json = String::new();
    io::stdin().read_to_string(&mut json).chain_err(
        || "could not read stdin",
    )?;

    let document = JsonApiDocument::from_str(&json).chain_err(
        || "could not read input as JSON API",
    )?;
    rustdoc_static::render_docs(&document, output_path)?;

    Ok(())
}

quick_main!(run);
