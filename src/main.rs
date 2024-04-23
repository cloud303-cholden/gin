extern crate clap;

use std::env;
use std::process::Command;

use clap::{App, Arg, SubCommand};

fn main() {
	let app_m = App::new("gin")
		.arg(
            Arg::with_name("profile")
            .takes_value(true)
            .long("profile")
            .short('p')
        )
		.subcommand(SubCommand::with_name("config"))
		.get_matches();

	match app_m.subcommand() {
		Some(("config", _)) => {
			println!("The config subcommand is not implemented!")
		}
		_ => {
			let profile = match app_m.get_one::<String>("profile") {
				None => env::var("GIT_DEFAULT_PROFILE").unwrap_or_else(|e| {
					panic!(
						"No profile supplied and could not find GIT_DEFAULT_PROFILE: {}",
						e
					)
				}),
				Some(val) => val.to_string(),
			};

            // TODO: Implement git init
			// Command::new("git")
			//     .arg("init")
			//     .spawn()
			//     .expect("Git failed to initialize.");

			let config_home = env::var("XDG_CONFIG_HOME")
				.unwrap_or_else(|e| panic!("Could not find XDG_CONFIG_HOME: {}", e));
			let git_config = config_home + "/git/" + &profile + ".conf";

			let output = Command::new("git")
				.arg("config")
				.arg("-f")
				.arg(git_config)
				.arg("--list")
				.output()
				.expect("Failed to copy Git config");

			let string_output: String = String::from_utf8_lossy(&output.stdout).into();
			let res: Vec<String> = string_output.lines().map(|s| s.to_string()).collect();
			for opt in &res {
				let mut opts = opt.split('=');
				let key = match opts.next() {
					Some(opt_key) => opt_key,
					None => continue,
				};
				let val = match opts.next() {
					Some(opt_val) => opt_val,
					None => continue,
				};
				Command::new("git")
					.arg("config")
					.arg(key)
					.arg(val)
					.spawn()
					.expect("Failed to set config")
                    .wait()
					.expect("Failed to exit");
			}
		}
	}
}
