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
            (about: "Get a password")
            (@arg path: +required "Password path to get")
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
    match repo.get(path) {
        Ok(p) => {
            println!("{}", String::from_utf8(p).chain_err(|| "could not parse password as UTF-8")?);
        },
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

fn run_insert(matches: &ArgMatches, repo: &mut Repository) -> Result<()> {
    let path = matches.value_of("path").ok_or("missing path argument".to_string())?;
    let password = matches.value_of("password").ok_or("missing password argument".to_string())?;
    repo.insert(path, password.as_bytes()).chain_err(|| "failed to insert password")?;
    Ok(())
}

fn run_delete(matches: &ArgMatches, repo: &mut Repository) -> Result<()> {
    let path = matches.value_of("path").ok_or("missing path argument".to_string())?;
    repo.delete(path).chain_err(|| "failed to delete password from repository")?;
    Ok(())
}
