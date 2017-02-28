extern crate error_chain;
extern crate serde;
extern crate rmp_serde;
extern crate sodiumoxide;
extern crate xdg;

extern crate mine;


use std::fs::File;

use mine::MineKey;
use ::errors::*;


pub fn init_run(dirs: xdg::BaseDirectories) -> Result<()> {
    let key_path = dirs.place_data_file("secret.key")
        .chain_err(|| "cannot place secret key")?;

    let key: MineKey = MineKey::new();

    let mut f = File::create(&key_path)
        .chain_err(|| "unable to create key file")?;
    rmp_serde::encode::write(&mut f, &key)
        .chain_err(|| "failed to serialize key to disk")?;

    println!("==> Wrote secret key to '{}'", key_path.display());

    Ok(())
}
