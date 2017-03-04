extern crate error_chain;
extern crate serde;
extern crate rmp_serde;
extern crate sodiumoxide;
extern crate xdg;

extern crate mine;


use mine::Mine;
use ::errors::*;


pub fn init_run(mut mine: Mine) -> Result<()> {
    mine.generate_key()?;
    mine.save()?;

    println!("==> Wrote secret key to disk");

    Ok(())
}
