#[macro_use] extern crate error_chain;
#[macro_use] extern crate clap;
extern crate xdg;
extern crate libmine;


mod errors {
    use super::libmine;
    error_chain! {
        links {
            Libmine(libmine::Error, libmine::ErrorKind);
        }
    }
}


use clap::ArgMatches;

use libmine::repository::Repository;
use errors::*;


quick_main!(run);

fn run() -> Result<()> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("mine").chain_err(|| "could not create XDG instance")?;
    let repo_dir = xdg_dirs.create_data_directory("repository").chain_err(|| "could not create repository directory")?;
    let index_path = xdg_dirs.place_data_file("index").chain_err(|| "could not create index file path")?;

    let mut repo = libmine::load_repository(&repo_dir, &index_path)?;

    let app = clap_app!(mine =>
        (@subcommand get =>
            (about: "Get an entry")
            (@arg path: +required "Entry path to get")
            (@arg tag: "Entry tag to fetch")
        )
        (@subcommand insert =>
            (about: "Insert a password")
            (@arg path: +required "Path to insert password under")
            (@arg password: +required "Password to insert")
        )
        (@subcommand delete =>
            (about: "Delete a password")
            (@arg path: +required "Path to password to delete")
        )
    );
    let matches = app.get_matches();

    match matches.subcommand() {
        ("get",    Some(sub_m)) => run_get(sub_m, &mut repo)?,
        ("insert", Some(sub_m)) => run_insert(sub_m, &mut repo)?,
        ("delete", Some(sub_m)) => run_delete(sub_m, &mut repo)?,
        (_, _) => Err("no subcommand specified")?,
    }

    libmine::save_index(&repo, &index_path)?;
    Ok(())
}

fn run_get(matches: &ArgMatches, repo: &mut Repository) -> Result<()> {
    let path = matches.value_of("path").ok_or("missing path argument".to_string())?;
    let key = matches.value_of("tag").unwrap_or("password");
    let entry: libmine::Entry = repo.get(path).chain_err(|| "failed to get entry from repository")?;
    println!("{}", entry.get(key).chain_err(|| "no tag with that key")?);
    Ok(())
}

fn run_insert(matches: &ArgMatches, repo: &mut Repository) -> Result<()> {
    let path = matches.value_of("path").ok_or("missing path argument".to_string())?;
    let password = matches.value_of("password").ok_or("missing password argument".to_string())?;
    let mut entry = libmine::Entry::new();
    entry.insert("password".to_string(), password.to_string()).chain_err(|| "failed to set password tag")?;
    repo.insert(path, entry).chain_err(|| "failed to insert password")?;
    Ok(())
}

fn run_delete(matches: &ArgMatches, repo: &mut Repository) -> Result<()> {
    let path = matches.value_of("path").ok_or("missing path argument".to_string())?;
    repo.delete(path).chain_err(|| "failed to delete password from repository")?;
    Ok(())
}
