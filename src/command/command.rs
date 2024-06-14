use std::env;




pub struct CommandCollector {
	args: Vec<String>,
}

impl CommandCollector {

	pub fn new() -> Self {
		CommandCollector {
			args: env::args().collect(),
		}
	}

	pub fn get(&self, index: usize) -> &str {
		if index < self.args.len() {
			return &self.args[index];
		}
		""
	}
	
}