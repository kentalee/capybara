pub mod daemon;
pub mod serve;
pub mod version;
pub trait Entry {
	fn name(&self) -> String;
	fn register_command(&self, _: clap::Command) -> clap::Command;
	fn handle_command(&self, _: &clap::ArgMatches);
}
