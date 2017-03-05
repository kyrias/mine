use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;


struct Ignore;

impl<E> From<E> for Ignore where E: Error {
    fn from(_: E) -> Ignore {
        Ignore
    }
}

fn main() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    let version = match commit_info() {
        Some(v) => v,
        None => env!("CARGO_PKG_VERSION").to_owned(),
    };
    File::create(out_dir.join("version-info.txt"))
        .unwrap()
        .write_all(version.as_bytes())
        .unwrap();
}

fn commit_info() -> Option<String> {
    match (commit_hash(), commit_date()) {
        (Ok(hash), Ok(date)) => Some(format!("{} ({})", hash.trim_right(), date)),
        _ => None,
    }
}

fn commit_hash() -> Result<String, Ignore> {
    let tag = Command::new("git")
        .args(&["describe", "--tags"])
        .output()?
        .stdout;
    if !tag.is_empty() {
        Ok(String::from_utf8(tag)?)
    }  else {
        Err(Ignore)
    }
}

fn commit_date() -> Result<String, Ignore> {
    Ok(String::from_utf8(Command::new("git")
                         .args(&["log", "-1", "--date=short", "--pretty=format:%cd"])
                         .output()?
                         .stdout)?)
}
