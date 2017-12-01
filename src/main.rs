#[macro_use] extern crate error_chain;
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



use libmine::repository::Repository;
use errors::*;


quick_main!(run);

fn run() -> Result<()> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("mine").chain_err(|| "could not create XDG instance")?;
    let repo_dir = xdg_dirs.create_data_directory("repository").chain_err(|| "could not create repository directory")?;
    let index_path = xdg_dirs.place_data_file("index").chain_err(|| "could not create index file path")?;
    let mut repo = load_repository(&repo_dir, &index_path)?;
    repo.insert("foo/bar", &[1,2,3,4]).chain_err(|| "could not insert 'foo/bar'")?;
    println!("{:?}", repo.get("foo/bar").chain_err(|| "could not get 'foo/bar'")?);
    println!("{:?}", repo.list("foo").chain_err(|| "could not list 'foo'")?);
    save_index(&repo, &index_path)?;
    Ok(())
}


}

    Ok(())
}
