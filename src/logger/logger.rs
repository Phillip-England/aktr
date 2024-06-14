
use colored::*;

pub struct AktrLogger {}

impl AktrLogger {

    pub fn new() -> Self {
        AktrLogger {}
    }

    pub fn err(self: &Self, message: String) {
        println!("{}", format!("Error: {}", message).red().bold());
    }

    pub fn info(self: &Self, message: &str) {
        println!("{}", format!("Info: {}", message).green().bold());
    }

    pub fn warn(self: &Self, message: &str) {
        println!("{}", format!("Warning: {}", message).yellow().bold());
    }

}