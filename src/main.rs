// `error_chain` recursion adjustment
#![recursion_limit = "1024"]
// Make rustc's built-in lints more strict
#![warn(warnings)]

extern crate colored;
use colored::*;

extern crate clap;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;
extern crate mime;
extern crate rusty_secrets;

mod errors;
use errors::*;

mod input;
use input::Input;

mod cli;
mod cmds;
mod version;
mod logger;
use logger::ColoredTermLogger;

use log::{Level, LevelFilter};
use std::path::Path;

fn main() {
    if let Err(ref e) = run() {
        error!("{}", e);

        for e in e.iter().skip(1) {
            error!("{} {}", "reason:".yellow().bold(), e);
        }

        if let Some(backtrace) = e.backtrace() {
            error!("{} {:?}", "backtrace:".blue().bold(), backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let app = cli::build_cli().version(version::get());
    let matches = app.get_matches();

    let verbose = matches
        .subcommand_matches("split")
        .or_else(|| matches.subcommand_matches("recover"))
        .map(|matches| matches.is_present("verbose"))
        .unwrap_or(false);

    let level = if verbose { Level::Debug } else { Level::Info };
    let logger = ColoredTermLogger::with_level(level);

    log::set_max_level(LevelFilter::Debug);
    log::set_boxed_logger(Box::new(logger)).unwrap();

    if let Some(matches) = matches.subcommand_matches("split") {
        let secret_arg = matches.value_of("INPUT").unwrap();
        let secret_input = if secret_arg == "-" {
            Input::stdin()
        } else {
            Input::file(secret_arg.to_string()).chain_err(|| ErrorKind::CannotOpenSecretFile(secret_arg.to_string()))?
        };

        let output_path = Path::new(matches.value_of("DIR").unwrap());
        let k = matches.value_of("k").unwrap().parse::<u8>().unwrap();
        let n = matches.value_of("n").unwrap().parse::<u8>().unwrap();
        let mime_type = matches.value_of("MIME").map(|v| v.parse().unwrap());
        let sign_shares = matches.is_present("sign");
        let raw = matches.is_present("raw");
        let share_tmpl = matches.value_of("share-tmpl").unwrap_or("share_{{num}}");

        cmds::split(
            secret_input,
            output_path,
            k,
            n,
            mime_type,
            sign_shares,
            raw,
            share_tmpl,
        )?
    } else if let Some(matches) = matches.subcommand_matches("recover") {
        let shares = matches
            .values_of("SHARES")
            .unwrap()
            .map(Path::new)
            .collect();

        let output_path = matches.value_of("FILE").map(Path::new);
        let verify_signatures = matches.is_present("verify");
        let raw = matches.is_present("raw");

        cmds::recover(shares, output_path, verify_signatures, raw)?
    }

    Ok(())
}
