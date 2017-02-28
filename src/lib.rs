#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

extern crate xdg;
extern crate serde;
extern crate sodiumoxide;
extern crate rmp_serde;


use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use serde::Deserialize;
use sodiumoxide::crypto::secretbox;


pub mod errors {
    extern crate rmp_serde;

    error_chain! {
        foreign_links {
            RmpSerde(rmp_serde::decode::Error);
        }
    }
}

use errors::*;


#[derive(Deserialize, Serialize, Debug)]
pub struct MineKey {
    pub key: secretbox::Key,
}

impl MineKey {
    pub fn new() -> MineKey {
        MineKey {
            key: secretbox::gen_key(),
        }
    }

    pub fn load_key<P: AsRef<Path>>(key_path: P) -> Result<MineKey> {
        let f = File::open(&key_path).unwrap();
        let mut de = rmp_serde::Deserializer::new(f);
        let key: MineKey = Deserialize::deserialize(&mut de)?;

        Ok(key)
    }

}


#[derive(Deserialize, Serialize, Debug)]
pub struct Password {
    pub password: String,
    pub tags: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Encrypted {
    pub nonce: secretbox::Nonce,
    pub ciphertext: Vec<u8>,
}


pub struct Mine {
    pub dirs: xdg::BaseDirectories,
    key: MineKey,
}

impl Mine {
    pub fn new(name: &str) -> Result<Mine> {
        let dirs = xdg::BaseDirectories::with_prefix(name).unwrap();

        let key_path = dirs.find_data_file("secret_key")
            .ok_or("could not find secret key")?;
        let key = MineKey::load_key(key_path.as_path())
            .chain_err(|| "failed to load private key")?;

        Ok(Mine {
            dirs: dirs,
            key: key,
        })
    }

    pub fn from_key(name: &str, key: MineKey) -> Result<Mine> {
        let dirs = xdg::BaseDirectories::with_prefix(name).unwrap();

        let key_path = dirs.place_data_file("secret_key")
            .chain_err(|| "could not place secret key")?;
        let mut f = File::create(&key_path)
            .chain_err(|| "failed to create secret key file")?;

        rmp_serde::encode::write(&mut f, &key)
            .chain_err(|| "failed to serialize key to disk")?;

        Mine::new(name)
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Encrypted {
        let nonce = secretbox::gen_nonce();
        let ciphertext = secretbox::seal(plaintext, &nonce, &self.key.key);
        Encrypted {
            nonce: nonce,
            ciphertext: ciphertext,
        }
    }

    pub fn decrypt(&self, ciphertext: &Encrypted) -> Result<Vec<u8>> {
        match secretbox::open(&ciphertext.ciphertext, &ciphertext.nonce, &self.key.key) {
            Ok(plaintext) => Ok(plaintext),
            Err(()) => Err("failed to decrypt".to_owned())?,
        }
    }
}
