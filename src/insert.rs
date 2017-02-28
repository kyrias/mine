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

use mine::{MineKey, Encrypted, Password};
use ::errors::*;


pub fn insert_run(matches: &ArgMatches, dirs: xdg::BaseDirectories) -> Result<()> {
    let key_path = dirs.find_data_file("secret.key").ok_or("could not find secret key")?;
    let key = MineKey::load_key(key_path.as_path())
        .chain_err(|| "could not load secret key")?;


    let password = Password {
        password: matches.value_of("PASSWORD").unwrap().to_owned(),
        tags: HashMap::new(),
    };
    let encoded = rmp_serde::encode::to_vec(&password)
        .chain_err(|| "failed to encode password struct")?;

    let encrypted: Encrypted = key.encrypt(&encoded[..]);


    let name = matches.value_of("NAME").unwrap();
    let pass_path = dirs.place_data_file(Path::new("store").join(name))
        .chain_err(|| "cannot place password file")?;


    let mut f = File::create(&pass_path)
        .chain_err(|| "unable to create password file")?;
    rmp_serde::encode::write(&mut f, &encrypted)
        .chain_err(|| "failed to serialize encrypted password to disk")?;

    println!("==> Wrote password to '{}'", pass_path.display());

    Ok(())
}
