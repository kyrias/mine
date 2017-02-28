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


mod errors {
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

    pub fn encrypt(&self, plaintext: &[u8]) -> Encrypted {
        let nonce = secretbox::gen_nonce();
        let ciphertext = secretbox::seal(plaintext, &nonce, &self.key);
        Encrypted {
            nonce: nonce,
            ciphertext: ciphertext,
        }
    }

    pub fn decrypt(&self, ciphertext: &Encrypted) -> Result<Vec<u8>> {
        match secretbox::open(&ciphertext.ciphertext, &ciphertext.nonce, &self.key) {
            Ok(plaintext) => Ok(plaintext),
            Err(()) => Err("failed to decrypt".to_owned())?,
        }
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
