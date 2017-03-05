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
mod set_tag;


use clap::{Arg, App, AppSettings, SubCommand};

use mine::Mine;


mod errors {
    extern crate mine;
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
        links {
            MineLib(mine::errors::Error, mine::errors::ErrorKind);
        }
    }
}

use errors::*;


quick_main!(run);


fn run() -> Result<()> {
    sodiumoxide::init();

    let matches = App::new("mine")
        .version("0.0.0")
        .author("Johannes Löthberg <johannes@kyriasis.com>")
        .about("NaCL based password manager in Rust")
        .global_settings(&[AppSettings::ColoredHelp])
        .setting(AppSettings::SubcommandRequiredElseHelp)
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
        .subcommand(SubCommand::with_name("set-tag")
                    .about("set a tag on a password")
                    .arg(Arg::with_name("PASSWORD")
                         .required(true)
                         .index(1))
                    .arg(Arg::with_name("TAG")
                         .required(true)
                         .index(2))
                    .arg(Arg::with_name("VALUE")
                         .required(true)
                         .index(3)))
        .get_matches();

    let mine = Mine::new("mine")
        .chain_err(|| "failed to initialize mine")?;

    let subcommand = matches.subcommand_name().unwrap();
    let sub_matches = matches.subcommand_matches(subcommand).unwrap();

    match subcommand {
        "init" => init::init_run(mine)?,
        "insert" => insert::insert_run(mine, sub_matches)?,
        "show" => show::show_run(mine, sub_matches)?,
        "set-tag" => set_tag::set_tag_run(mine, sub_matches)?,
        _ => unreachable!(),
    }

    Ok(())
}

