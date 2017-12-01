#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate sequence_trie;
extern crate rand;


pub mod repository;
mod errors {
    error_chain! {}
}


pub use errors::*;
