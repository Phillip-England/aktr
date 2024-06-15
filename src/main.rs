
mod config;
mod logger;
mod command;

use config::aktr_config::AktrConfig;
use config::aktr_config::DEFAULT_AKTR_CONFIG_PATH;
use config::package_json::PackageJson;
use logger::logger::AktrLogger;
use command::command::CommandCollector;




fn main() {

	let commands = CommandCollector::new();

    if commands.get(1) == "init" {
		handle_init();
		return;
    }

    if commands.get(1) == "generate" {
		handle_generate();
		return;
    }

	if commands.get(1) == "" {
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
    AktrLogger::skip_line();
    AktrLogger::title_line("aktr - a rust-powered typescript meta-framework");
	let commands = CommandCollector::new();
	let mut third_command = commands.get(2); 
	if third_command == "" {
		third_command = DEFAULT_AKTR_CONFIG_PATH;
	}
	let aktr_config_dest = format!("./{}/{}", third_command, DEFAULT_AKTR_CONFIG_PATH);
	AktrLogger::indented_line(format!("creating {}", DEFAULT_AKTR_CONFIG_PATH).as_str());
	let default_config = AktrConfig::default();
	let config = default_config.init_config_file(aktr_config_dest.to_string());
	if config.is_err() {
		let err = config.err().unwrap();
		AktrLogger::err(err.to_string());
		return;
	}
    let config = config.unwrap();
	AktrLogger::indented_line(format!("{} created", DEFAULT_AKTR_CONFIG_PATH).as_str());
	if third_command != DEFAULT_AKTR_CONFIG_PATH {
		AktrLogger::indented_line(format!("run 'cd {}', then 'bun install' to install dependencies", third_command).as_str());
	} else {
		AktrLogger::indented_line("run 'bun install' to install dependencies");
	}
    AktrLogger::indented_line("finally, run 'aktr generate' to generate typescript code from rust");
    let package_json = PackageJson::default();
    let package_json_write = package_json.write_to_file(format!("./{}/{}", third_command, config.src.package_json).as_str());
    if package_json_write.is_err() {
        let err = package_json_write.err().unwrap();
        AktrLogger::err(err.to_string());
        return;
    } 
    AktrLogger::skip_line();
}

fn handle_generate() {
	let commands = CommandCollector::new();
	AktrLogger::info("generating aktr output files");
	let config = AktrConfig::from_file();
	if config.is_err() {
		let err = config.err().unwrap();
		AktrLogger::err(err.to_string());
		return;
	}
	let config = config.unwrap();
	let src_dir = config.src.dir;
	AktrLogger::info(format!("src directory: {}", src_dir).as_str());
}