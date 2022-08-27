extern crate clap;

use std::env;
use std::process::Command;


use clap::{
    App,
    Arg,
    SubCommand,
};

fn main() {
    let app_m = App::new("gin")
        .arg(Arg::with_name("profile"))
        .subcommand(SubCommand::with_name("config"))
        .get_matches();

    match app_m.subcommand() {
        Some(("config", _)) => {println!("The config subcommand is not implemented!")},
        _ => {
            let profile = match app_m.get_one::<String>("profile") {
                None => {
                    env::var("GIT_DEFAULT_PROFILE").unwrap_or_else(|e| {
                        panic!("No profile supplied and could not find GIT_DEFAULT_PROFILE: {}", e)
                    })
                },
                Some(val) => val.to_string(),
            };

            Command::new("git")
                .arg("init")
                .spawn()
                .expect("Git failed to initialize.");

            let config_home = env::var("XDG_CONFIG_HOME").unwrap_or_else(|e| {
                panic!("Could not find XDG_CONFIG_HOME: {}", e)
            });
            let git_config = config_home + "/git/" + &profile + ".conf";
            Command::new("cp")
                .arg(git_config)
                .arg("./.git/config")
                .spawn()
                .expect("Failed to copy Git config.");
        }
    }

}
