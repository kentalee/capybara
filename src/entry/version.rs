use rust_i18n::t;
use yaml_rust2::{Yaml, yaml::Hash as YamlHash, YamlEmitter};

use crate::define::{args, dict};
use crate::entry::Entry as EntryTrait;

include!(concat!(env!("OUT_DIR"), "/built.rs"));

mod daemon {}

pub struct Entry {}

pub fn new() -> impl EntryTrait {
	Entry {}
}

impl EntryTrait for Entry {
	fn name(&self) -> String { dict::CMD_VERSION.to_string() }

	fn register_command(&self, cmd: clap::Command) -> clap::Command {
		cmd.subcommand(clap::Command::new(dict::CMD_VERSION)
			.about(t!(dict::CMD_VERSION_ABOUT_TAG).into_owned())
			.arg(args::new_verbose()))
	}

	fn handle_command(&self, args: &clap::ArgMatches) {
		println!(
			"# {} version {} {}/{}",
			PKG_NAME, PKG_VERSION, CFG_TARGET_ARCH, CFG_OS
		);

		let detailed = args.get_flag(dict::ARG_VERBOSE);

		if !detailed {
			return;
		}

		let mut doc = YamlHash::new();

		doc.insert(Yaml::String("package".to_string()), (|| {
			let mut section = YamlHash::new();
			section.insert(Yaml::String("license".to_string()), Yaml::String(PKG_LICENSE.to_string()));
			section.insert(Yaml::String("authors".to_string()), Yaml::String(PKG_AUTHORS.to_string()));
			section.insert(Yaml::String("description".to_string()), Yaml::String(PKG_DESCRIPTION.to_string()));
			section.insert(Yaml::String("homepage".to_string()), Yaml::String(PKG_HOMEPAGE.to_string()));
			section.insert(Yaml::String("repository".to_string()), Yaml::String(PKG_REPOSITORY.to_string()));
			Yaml::Hash(section)
		})());

		doc.insert(Yaml::String("build".to_string()), (|| {
			let mut section = YamlHash::new();
			section.insert(Yaml::String("ci_platform".to_string()), Yaml::String(CI_PLATFORM.unwrap_or("None").to_string()));
			section.insert(Yaml::String("target".to_string()), Yaml::String(TARGET.to_string()));
			section.insert(Yaml::String("host".to_string()), Yaml::String(HOST.to_string()));
			section.insert(Yaml::String("profile".to_string()), Yaml::String(PROFILE.to_string()));
			Yaml::Hash(section)
		})());

		doc.insert(Yaml::String("version".to_string()), (|| {
			let mut section = YamlHash::new();
			section.insert(Yaml::String("os".to_string()), Yaml::String(CFG_OS.to_string()));
			section.insert(Yaml::String("arch".to_string()), Yaml::String(CFG_TARGET_ARCH.to_string()));
			section.insert(Yaml::String("major".to_string()), Yaml::String(PKG_VERSION_MAJOR.to_string()));
			section.insert(Yaml::String("minor".to_string()), Yaml::String(PKG_VERSION_MINOR.to_string()));
			section.insert(Yaml::String("patch".to_string()), Yaml::String(PKG_VERSION_PATCH.to_string()));
			section.insert(Yaml::String("pre".to_string()), Yaml::String(PKG_VERSION_PRE.to_string()));
			Yaml::Hash(section)
		})());

		doc.insert(Yaml::String("compiler".to_string()), (|| {
			let mut section = YamlHash::new();
			section.insert(Yaml::String("rustc".to_string()), Yaml::String(RUSTC_VERSION.to_string()));
			section.insert(Yaml::String("rustdoc".to_string()), Yaml::String(RUSTDOC_VERSION.to_string()));
			section.insert(Yaml::String("features".to_string()), Yaml::String(FEATURES_STR.to_string()));
			Yaml::Hash(section)
		})());

		doc.insert(Yaml::String("optimization".to_string()), (|| {
			let mut section = YamlHash::new();
			section.insert(Yaml::String("level".to_string()), Yaml::String(OPT_LEVEL.to_string()));
			section.insert(Yaml::String("debug".to_string()), Yaml::String(DEBUG.to_string()));
			section.insert(Yaml::String("number_of_jobs".to_string()), Yaml::String(NUM_JOBS.to_string()));
			Yaml::Hash(section)
		})());

		doc.insert(Yaml::String("configuration".to_string()), (|| {
			let mut section = YamlHash::new();
			section.insert(Yaml::String("architecture".to_string()), Yaml::String(CFG_TARGET_ARCH.to_string()));
			section.insert(Yaml::String("endianness".to_string()), Yaml::String(CFG_ENDIAN.to_string()));
			section.insert(Yaml::String("environment".to_string()), Yaml::String(CFG_ENV.to_string()));
			section.insert(Yaml::String("family".to_string()), Yaml::String(CFG_FAMILY.to_string()));
			section.insert(Yaml::String("os".to_string()), Yaml::String(CFG_OS.to_string()));
			section.insert(Yaml::String("pointer_width".to_string()), Yaml::String(CFG_POINTER_WIDTH.to_string()));
			Yaml::Hash(section)
		})());

		let mut detailed = String::new();
		YamlEmitter::new(&mut detailed).dump(&Yaml::Hash(doc)).unwrap();

		print!("{}", detailed)
	}
}



