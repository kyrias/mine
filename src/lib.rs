#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

extern crate xdg;
extern crate serde;
extern crate sodiumoxide;
extern crate rmp_serde;


use std::fs::File;
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
    key: Option<secretbox::Key>,
}

impl Mine {
    pub fn new(name: &str) -> Result<Mine> {
        let dirs = xdg::BaseDirectories::with_prefix(name).unwrap();

        Ok(Mine {
            dirs: dirs,
            key: None,
        })
    }

    pub fn generate_key(&mut self) -> Result<()> {
        match self.key {
            Some(_) => Err("Mine instance already has a key".to_owned())?,
            None => {
                let key = secretbox::gen_key();
                self.key = Some(key);
                Ok(())
            }
        }
    }

    pub fn save(&self) -> Result<()> {
        let key_path = self.dirs
            .place_data_file("secret_key")
            .chain_err(|| "could not place secret key")?;
        let mut f = File::create(&key_path)
            .chain_err(|| "failed to create secret key file")?;

        rmp_serde::encode::write(&mut f, &self.key)
            .chain_err(|| "failed to serialize key to disk")?;

        Ok(())
    }

    pub fn load(&mut self) -> Result<()> {
        let key_path = self.dirs
            .find_data_file("secret_key")
            .ok_or("could not find secret key")?;

        let f = File::open(&key_path).unwrap();
        let mut de = rmp_serde::Deserializer::new(f);
        let key: secretbox::Key = Deserialize::deserialize(&mut de)
            .chain_err(|| "failed to deserialize secret key")?;

        self.key = Some(key);

        Ok(())
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Encrypted> {
        let nonce = secretbox::gen_nonce();
        let key = self.key.as_ref().ok_or("no secret key available")?;
        let ciphertext = secretbox::seal(plaintext, &nonce, &key);
        Ok(Encrypted {
            nonce: nonce,
            ciphertext: ciphertext,
        })
    }

    pub fn decrypt(&self, ciphertext: &Encrypted) -> Result<Vec<u8>> {
        let key = self.key.as_ref().ok_or("no secret key available")?;
        match secretbox::open(&ciphertext.ciphertext, &ciphertext.nonce, &key) {
            Ok(plaintext) => Ok(plaintext),
            Err(()) => Err("failed to decrypt".to_owned())?,
        }
    }
}
