use rust_i18n::{i18n, t};

use crate::entry::Entry;

mod define;
mod service;
mod entry;

i18n!();

fn main() {
	let entries: Vec<Box<dyn Entry>> = vec![
		Box::new(entry::daemon::new()),
		Box::new(entry::serve::new()),
		Box::new(entry::version::new())
	];

	let mut command = clap::Command::new(entry::version::PKG_NAME)
		.about(t!(define::dict::APP_ABOUT_TAG).into_owned())
		.subcommand_required(true)
		.arg_required_else_help(true);

	for entry in &entries {
		command = entry.register_command(command)
	}

	let matched = command.get_matches();
	let (sub_command, args) = matched.subcommand().unwrap_or_else(|| {
		unreachable!()
	});

	if let Some(matched_entry) = entries.iter().find(
		|entry| { entry.name() == sub_command }
	) {
		matched_entry.handle_command(args);
	} else {
		unreachable!()
	}
}
