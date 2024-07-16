use clap::{Arg, ArgAction};
use rust_i18n::t;

use crate::define::dict;

pub fn new_config() -> Arg {
	Arg::new(dict::ARG_CONFIG)
		.short(dict::ARG_CONFIG_SHORT).long(dict::ARG_CONFIG)
		.help(t!(dict::ARG_CONFIG_ABOUT_TAG).into_owned())
		.num_args(1..).action(ArgAction::Append)
}

pub fn new_verbose() -> Arg {
	Arg::new(dict::ARG_VERBOSE)
		.short(dict::ARG_VERBOSE_SHORT).long(dict::ARG_VERBOSE)
		.help(t!(dict::ARG_VERBOSE_ABOUT_TAG).into_owned())
		.action(ArgAction::SetTrue)
}