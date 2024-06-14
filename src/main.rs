use colored::*;
use std::{env, path::PathBuf};

mod config;
mod logger;
mod command;

use config::config::{AktrConfig, DEFAULT_CONFIG_PATH};
use logger::logger::AktrLogger;
use command::command::CommandCollector;




fn main() {

	let args = CommandCollector::new();

    if args.get(1) == "init" {
		handle_init();
		return;
    }

    if args.get(1) == "generate" {
		handle_generate();
		return;
    }

	if args.get(1) == "" {
		println!("");
		AktrLogger::title_line("aktr - a rust-powered typescript meta-framework");
		AktrLogger::skip_line();
		AktrLogger::title_line("usage:");
		AktrLogger::indented_line("aktr init [path]    create a .aktr.toml file in the current directory or in the specified path");
		AktrLogger::indented_line("aktr generate       generate typescript code from rust");
		return;
	
	}


}


fn handle_init() {
	let args = CommandCollector::new();
	let mut third_arg = args.get(2); 
	if third_arg == "" {
		third_arg = DEFAULT_CONFIG_PATH;
	}
	let init_destination = format!("./{}/{}", third_arg, DEFAULT_CONFIG_PATH);

	AktrLogger::general("init", format!("creating {}", DEFAULT_CONFIG_PATH).as_str());
	let config = AktrConfig::default()
		.init_config_file(init_destination.to_string());
	if config.is_err() {
		let err = config.err().unwrap();
		AktrLogger::err(err.to_string());
		return;
	}
	AktrLogger::general("init", format!("{} created", DEFAULT_CONFIG_PATH).as_str());
	if third_arg != DEFAULT_CONFIG_PATH {
		AktrLogger::general("init", format!("run 'cd {}' then 'aktr generate' to build your application", third_arg).as_str());
	} else {
		AktrLogger::general("init", "run 'aktr generate' to build your application");

	}
}

fn handle_generate() {
	let args = CommandCollector::new();
	AktrLogger::info("generating aktr output files");
	let config = AktrConfig::from_file();
	if config.is_err() {
		let err = config.err().unwrap();
		AktrLogger::err(err.to_string());
		return;
	}
	let config = config.unwrap();
	let output_dir = config.output.dir;
	AktrLogger::info(format!("output directory: {}", output_dir).as_str());
}