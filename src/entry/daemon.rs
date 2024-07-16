use rust_i18n::t;

use crate::define::{args, dict};
use crate::entry::Entry as EntryTrait;

mod daemon {}

pub struct Entry {}

pub fn new() -> impl EntryTrait {
	Entry {}
}

impl EntryTrait for Entry {
	fn name(&self) -> String { dict::CMD_DAEMON.to_string() }

	fn register_command(&self, cmd: clap::Command) -> clap::Command {
		cmd.subcommand(clap::Command::new(dict::CMD_DAEMON)
			.about(t!(dict::CMD_DAEMON_ABOUT_TAG).into_owned())
			.arg(args::new_config()))
	}

	fn handle_command(&self, _: &clap::ArgMatches) {}
}
