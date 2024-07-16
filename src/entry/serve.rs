use rust_i18n::t;

use crate::define::{args, dict};
use crate::entry::Entry as EntryTrait;

mod daemon {}

pub struct Entry {}

pub fn new() -> impl EntryTrait {
	Entry {}
}

impl EntryTrait for Entry {
	fn name(&self) -> String { dict::CMD_SERVE.to_string() }

	fn register_command(&self, cmd: clap::Command) -> clap::Command {
		cmd.subcommand(clap::Command::new(dict::CMD_SERVE)
			.about(t!(dict::CMD_SERVE_ABOUT_TAG).into_owned())
			.arg(args::new_config()))
	}

	fn handle_command(&self, args: &clap::ArgMatches) {
		let path_list: Vec<&String> = args.get_many::<String>(dict::ARG_CONFIG)
			.expect(t!(dict::ERR_ARG_REQUIRED,  name = dict::ARG_CONFIG).into_owned().as_str())
			.collect();

		println!("{:#?}", path_list);
	}
}
