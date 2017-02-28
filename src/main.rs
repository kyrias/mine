#[macro_use]
extern crate error_chain;

extern crate clap;
extern crate serde;
extern crate rmp_serde;
extern crate sodiumoxide;
extern crate xdg;

extern crate mine;


mod init;


use clap::{App, AppSettings, SubCommand};


mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! { }
}

use errors::*;


quick_main!(run);


fn run() -> Result<()> {
    sodiumoxide::init();

    let matches = App::new("mine")
                    .version("0.0.0")
                    .author("Johannes Löthberg <johannes@kyriasis.com>")
                    .about("NaCL based password manager in Rust")
                    .setting(AppSettings::SubcommandRequired)
                    .subcommand(SubCommand::with_name("init")
                                .about("initialize password store"))
                    .get_matches();

    let subcommand = matches.subcommand_name().unwrap();

    let dirs = xdg::BaseDirectories::with_prefix("mine").unwrap();
    match subcommand {
        "init" => init::init_run(dirs)?,
        _ => unreachable!(),
    }

    Ok(())
}

