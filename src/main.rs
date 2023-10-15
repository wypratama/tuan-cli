use std::ffi::OsString;

use clap::{arg, Command};

mod cmd;
mod conf;
mod git;

fn cli() -> Command {
    Command::new("tuan")
        .about("CLI Program to help manage env")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Wicaksana Pratama<wicaksanapratama@gmail.com>")
        .subcommand(
            Command::new("minta")
                .about("Ask for help to tuan")
                .subcommand(
                    Command::new("env")
                        .about("Get environment variables from remote source")
                        .arg(arg!([SOURCE] "Remote source to read from")),
                )
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("minta", sub_matches)) => {
            let minta_command = sub_matches.subcommand();

            match minta_command {
                Some(("env", subsub_matches)) => {
                    let source = subsub_matches.get_one::<String>("SOURCE").unwrap();

                    match source.as_str() {
                        "local" => {
                            println!("Setting up local environment.");
                            let config = conf::read();
                            git::clone(&config.local.source, &config.local.branch);
                        }
                        "staging" => {
                            println!("Setting up staging environment.");
                            let config = conf::read();
                            git::clone(&config.staging.source, &config.staging.branch);
                        }
                        "production" => {
                            println!("Setting up production environment.");
                            let config = conf::read();
                            git::clone(&config.local.source, &config.local.branch);
                        }
                        _ => {
                            println!("Environment not recognized")
                        }
                    }
                }
                _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
            }
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Calling out to {ext:?} with {args:?}");
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}
