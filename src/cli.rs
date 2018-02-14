use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("RustySecrets CLI")
        .version("0.2-pre")
        .author("SpinResearch")
        .about("Split a secret of an arbitrary length in n different shares and k-out-of-n shares are required to recover it.")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::UnifiedHelpMessage)
        .subcommand(SubCommand::with_name("split")
                    .about("Split a secret into shares")
                    .visible_alias("s")
                    .setting(AppSettings::ColoredHelp)
                    .setting(AppSettings::UnifiedHelpMessage)
                    .arg(Arg::with_name("verbose")
                         .short("v")
                         .long("verbose")
                         .help("Enable verbose mode"))
                    .arg(Arg::with_name("k")
                         .short("k")
                         .required(true)
                         .takes_value(true)
                         .validator(validators::num::strictly_positive)
                         .help("Number of shares necessary to recover the secret"))
                    .arg(Arg::with_name("n")
                         .short("n")
                         .required(true)
                         .takes_value(true)
                         .validator(validators::num::strictly_positive)
                         .help("Total number of generated shares"))
                    .arg(Arg::with_name("share-tmpl")
                         .short("t")
                         .long("share-tmpl")
                         .takes_value(true)
                         .validator(validators::rusty_secrets::share_tmpl)
                         .help("Template for the share names. Defaults to 'share_{{num}}'"))
                    .arg(Arg::with_name("DIR")
                         .short("o")
                         .long("output")
                         .required(true)
                         .takes_value(true)
                         .validator(validators::fs::directory)
                         .help("Path to the directory to output the shares to"))
                    .arg(Arg::with_name("MIME")
                         .short("m")
                         .long("mime")
                         .takes_value(true)
                         .validator(validators::mime_type)
                         .help("The MIME type of the secret"))
                    .arg(Arg::with_name("raw")
                         .short("r")
                         .long("raw")
                         .conflicts_with("MIME")
                         .help("Include this flag to generate raw shares (ie. without a MIME type)"))
                    .arg(Arg::with_name("sign")
                         .short("s")
                         .long("sign")
                         .help("Sign the shares"))
                    .arg(Arg::with_name("INPUT")
                         .required(true)
                         .validator(validators::fs::file_or_stdin)
                         .help("Path to the file containing the secret to split, or - to read from stdin")))
        .subcommand(SubCommand::with_name("recover")
                    .about("Recover the secret from the shares")
                    .visible_alias("r")
                    .setting(AppSettings::ColoredHelp)
                    .setting(AppSettings::UnifiedHelpMessage)
                    .arg(Arg::with_name("verbose")
                         .short("v")
                         .long("verbose")
                         .help("Enable verbose mode"))
                    .arg(Arg::with_name("SHARES")
                         .required(true)
                         .takes_value(true)
                         .multiple(true)
                         .validator(validators::fs::file)
                         .help("Paths to shares to recover the secret from"))
                    .arg(Arg::with_name("raw")
                         .short("r")
                         .long("raw")
                         .help("Include this flag if the shares are raw (ie. do not contain a MIME type)"))
                    .arg(Arg::with_name("verify")
                         .long("verify")
                         .help("Verify the shares signatures"))
                    .arg(Arg::with_name("FILE")
                         .short("o")
                         .long("output")
                         .takes_value(true)
                         .help("Path to file to output the secret to, prints to stdout if omitted")))
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub mod validators {

    pub mod rusty_secrets {

        pub fn share_tmpl(value: String) -> Result<(), String> {
            if !value.contains("{{num}}") {
                return Err("Invalid share template, must contain '{[num]}'".to_string());
            }

            Ok(())
        }

    }

    pub mod num {

        pub fn strictly_positive(value: String) -> Result<(), String> {
            let n = value.parse::<u8>();

            if n.is_err() {
                return Err(format!("{} is not a positive number", value));
            }

            if n.unwrap() < 1 {
                return Err(format!("{} is not strictly positive", value));
            }

            Ok(())
        }

    }

    pub fn mime_type(value: String) -> Result<(), String> {
        use mime::Mime;

        if value.parse::<Mime>().is_err() {
            return Err(format!("{} is not a valid MIME type", value));
        }

        Ok(())
    }

    pub mod fs {

        use std::path::Path;

        pub fn file(value: String) -> Result<(), String> {
            let path = Path::new(&value);

            if !path.exists() {
                return Err(format!("'{}' does not exists", value));
            }

            if !path.is_file() {
                return Err(format!("'{}' is not a file", value));
            }

            Ok(())
        }

        pub fn file_or_stdin(value: String) -> Result<(), String> {
            if value == "-" {
                return Ok(());
            }

            file(value)
        }

        pub fn directory(value: String) -> Result<(), String> {
            let path = Path::new(&value);

            if !path.exists() {
                return Err(format!("'{}' does not exists", value));
            }

            if !path.is_dir() {
                return Err(format!("'{}' is not a directory", value));
            }

            Ok(())
        }

    }
}
