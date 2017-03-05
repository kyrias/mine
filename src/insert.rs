extern crate error_chain;
extern crate serde;
extern crate rmp_serde;
extern crate sodiumoxide;
extern crate clap;
extern crate xdg;

extern crate mine;


use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use clap::ArgMatches;

use mine::{Mine, Encrypted, Password};
use ::errors::*;


pub fn insert_run(mine: Mine, matches: &ArgMatches) -> Result<()> {
    let password = Password {
        password: matches.value_of("PASSWORD").unwrap().to_owned(),
        tags: HashMap::new(),
    };
    let encoded = rmp_serde::encode::to_vec(&password)
        .chain_err(|| "failed to encode password struct")?;

    let encrypted: Encrypted = mine.encrypt(&encoded[..])
        .chain_err(|| "failed to encrypt password")?;


    let name = matches.value_of("NAME").unwrap();
    let pass_path = mine.dirs
        .place_data_file(Path::new("store").join(name))
        .chain_err(|| "cannot place password file")?;


    let mut f = File::create(&pass_path)
        .chain_err(|| "unable to create password file")?;
    rmp_serde::encode::write(&mut f, &encrypted)
        .chain_err(|| "failed to serialize encrypted password to disk")?;

    println!("==> Wrote password to '{}'", pass_path.display());

    Ok(())
}
