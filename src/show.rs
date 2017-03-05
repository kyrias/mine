extern crate error_chain;
extern crate serde;
extern crate rmp;
extern crate rmp_serde;
extern crate sodiumoxide;
extern crate clap;
extern crate xdg;

extern crate mine;


use std::fs::File;
use std::path::Path;

use clap::ArgMatches;

use mine::{Mine, Encrypted, Password};
use ::errors::*;


pub fn show_run(mine: Mine, matches: &ArgMatches) -> Result<()> {
    let name = matches.value_of("NAME").unwrap();
    let pass_path = mine.dirs
        .find_data_file(Path::new("store").join(name))
        .ok_or("cannot find password file")?;

    let f = File::open(&pass_path).chain_err(|| "unable to open password file")?;
    let encrypted: Encrypted = rmp_serde::decode::from_read(f)
        .chain_err(|| "could not deserialize password")?;

    let decrypted = mine.decrypt(&encrypted)
        .chain_err(|| "failed to decrypt password")?;
    let password: Password = rmp_serde::decode::from_read(&decrypted[..])
        .chain_err(|| "failed to decode password struct")?;

    println!("{}", password.password);
    for (key, value) in password.tags.iter() {
        println!("{} => {}", key, value);
    }

    Ok(())
}
