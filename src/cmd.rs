use fs_extra::dir::copy as excopy;
use fs_extra::dir::CopyOptions;
use std::fs;

pub fn copy(clone_path: &str) {
    // Specify the source and destination directories
    let source = clone_path.to_owned() + "/.git";

    // Remove .git folder before copy so it's not conflicting with monorepo
    fs::remove_dir_all(source).expect("Folder already exist and can't be deleted");

    let destination = "."; // Current directory

    let options = CopyOptions {
        content_only: true,
        copy_inside: true,
        overwrite: true,
        ..Default::default()
    };

    // Copy all files and folders from the source to the destination
    match excopy(clone_path, destination, &options) {
        Ok(_) => {
            println!("Files copied successfully.");
            println!("Cleaning up");
            cleanup(clone_path);
        }
        Err(err) => println!("Error: {}", err),
    }
}

pub fn cleanup(clone_path: &str) {
    let cleaned = fs::remove_dir_all(clone_path);

    match cleaned {
        Ok(_) => println!("Env files successfully copied"),
        Err(err) => println!("Error cleaning up: {}", err),
    }
}
