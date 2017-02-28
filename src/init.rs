extern crate error_chain;
extern crate serde;
extern crate rmp_serde;
extern crate sodiumoxide;
extern crate xdg;

extern crate mine;


use mine::{Mine, MineKey};
use ::errors::*;


pub fn init_run() -> Result<()> {
    let key: MineKey = MineKey::new();
    Mine::from_key("mine", key)?;

    println!("==> Wrote secret key to disk");

    Ok(())
}
