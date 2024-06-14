use colored::*;
use std::{env, path::PathBuf};

mod config;
mod logger;

use config::config::AktrConfig;
use logger::logger::AktrLogger;




fn main() {

    let args: Vec<String> = env::args().collect();
    let mut second_arg = "";
    if args.len() >= 2 { second_arg = &args[1]; }

    if second_arg == "init" {

		let mut third_arg = "";
		if args.len() >= 3 { 
			third_arg = &args[2]; 
		}

		if third_arg == "" {
			third_arg = "./.aktr.toml";
		}
		let init_destination = format!("./{}/.aktr.toml", third_arg);

        AktrLogger::new().info("creating .aktr.toml");
        let config = AktrConfig::default()
            .init_config_file(init_destination.to_string());
        if config.is_err() {
            let err = config.err().unwrap();
            AktrLogger::new().err(err.to_string());
            return;
        }
        AktrLogger::new().info(".aktr.toml created");
    }

    if second_arg == "generate" {
        AktrLogger::new().info("generating aktr output files");
        let config = AktrConfig::from_file();
        if config.is_err() {
            let err = config.err().unwrap();
            AktrLogger::new().err(err.to_string());
            return;
        }
        let config = config.unwrap();
        let output_dir = config.output.dir;
        AktrLogger::new().info(format!("output directory: {}", output_dir).as_str());
    }


}