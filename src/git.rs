use git2::{Cred, RemoteCallbacks};
use std::env;
use std::fs;
use std::path::Path;

use crate::cmd::copy;

pub fn clone(source: &str, branch: &str) {
    let clone_path = "/tmp/tuan";

    if Path::new(clone_path).exists() {
        fs::remove_dir_all(clone_path).expect("Folder already exist and can't be deleted")
    }

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
        )
    });

    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    // Clone the project.
    let res = builder.branch(branch).clone(source, Path::new(clone_path));

    match res {
        Ok(_) => copy(clone_path),
        Err(err) => println!("Error cloning source branch: {}", err),
    }
}
