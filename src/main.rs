use colored::*;
use std::env;

mod config;
mod logger;

use config::config::AktrConfig;
use logger::logger::AktrLogger;




fn main() {

    let args: Vec<String> = env::args().collect();
    let mut second_arg = "";
    let mut _third_arg = "";
    if args.len() >= 2 { second_arg = &args[1]; }
    if args.len() >= 3 { _third_arg = &args[2]; }

    if second_arg == "init" {
        AktrLogger::new().info("creating .aktr.toml");
        let config = AktrConfig::default()
            .init_config_file();
        if config.is_err() {
            let err = config.err().unwrap();
            AktrLogger::new().err(err.to_string());
            return;
        }
        AktrLogger::new().info(".aktr.toml created");
    }

    if second_arg == "install" {
        AktrLogger::new().info("installing aktr typescript files");
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