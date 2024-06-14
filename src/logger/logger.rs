
use colored::*;

pub struct AktrLogger {}

impl AktrLogger {

    pub fn new() -> Self {
        AktrLogger {}
    }

    pub fn err(message: String) {
        println!("{}", format!("Error: {}", message).red().bold());
    }

    pub fn info(message: &str) {
        println!("{}", format!("Info: {}", message).green().bold());
    }

    pub fn warn(message: &str) {
        println!("{}", format!("Warning: {}", message).yellow().bold());
    }

	pub fn general(title: &str, message: &str) {
		println!("{}", format!("{}: {}", title.cyan().bold(), message));
	}

	pub fn title_line(title: &str) {
		println!("{}", format!("{}", title.cyan().bold()));
	}

	pub fn indented_line(message: &str) {
		println!("{}", format!("  {}", message));
	}

	pub fn skip_line() {
		println!("");
	}

}