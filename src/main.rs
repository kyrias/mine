#[macro_use]
extern crate error_chain;

extern crate clap;
extern crate serde;
extern crate rmp_serde;
extern crate sodiumoxide;
extern crate xdg;

extern crate mine;


mod init;
mod insert;
mod show;


use clap::{Arg, App, AppSettings, SubCommand};


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
                    .subcommand(SubCommand::with_name("insert")
                                .about("insert a password")
                                .arg(Arg::with_name("NAME")
                                     .required(true)
                                     .index(1))
                                .arg(Arg::with_name("PASSWORD")
                                     .required(true)
                                     .index(2)))
                    .subcommand(SubCommand::with_name("show")
                                .about("show a password")
                                .arg(Arg::with_name("NAME")
                                     .help("Password name")
                                     .required(true)
                                     .index(1)))
                    .get_matches();

    let subcommand = matches.subcommand_name().unwrap();
    let sub_matches = matches.subcommand_matches(subcommand).unwrap();

    let dirs = xdg::BaseDirectories::with_prefix("mine").unwrap();
    match subcommand {
        "init" => init::init_run(dirs)?,
        "insert" => insert::insert_run(sub_matches, dirs)?,
        "show" => show::show_run(sub_matches, dirs)?,
        _ => unreachable!(),
    }

    Ok(())
}

