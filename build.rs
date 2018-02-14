
extern crate git_build_version;
extern crate mime;
extern crate clap;

use clap::Shell;

use std::env;
use std::path::Path;
use std::fs;

include!("src/cli.rs");

fn main() {
    gen_completions();
    expose_git_describe();
}

fn gen_completions() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let mut path_buf = Path::new(&out_dir).to_path_buf();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("rustysecrets-completions");
    let path = path_buf.as_path();

    if !path.exists() {
        fs::create_dir(path).unwrap();
    }

    let mut app = build_cli();
    app.gen_completions("rustysecrets", Shell::Bash, &path);
    app.gen_completions("rustysecrets", Shell::Zsh, &path);
    app.gen_completions("rustysecrets", Shell::Fish, &path);
    // app.gen_completions("rustysecrets", Shell::PowerShell, &outdir);
}

fn expose_git_describe() {
    git_build_version::write_version(".").expect("Saving git version");
}
