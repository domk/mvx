use clap::Parser;
use std::path::PathBuf;

const SEMVER: &str = "0.2.0";

#[derive(Parser)]
#[clap(author="Dominik Madon <dominik at acm.org>",
       name="Mvx",
       version=SEMVER,
       about,
       set_term_width(80))]
/// Move or copy files changing only their extension.
///
/// This utility program takes all given files and modify their
/// extension in a bulk mode. One can add an extension, remove one, or
/// change it although this feature may lack some good use case :-).
pub struct Args {
    /// Files to operate on
    #[clap(value_parser)]
    pub filenames: Vec<PathBuf>,

    /// Dry run, don't change filesystem
    #[clap(short = 'n', long, value_parser, default_value_t = false)]
    pub dry_run: bool,

    /// Dump a nroff man page
    #[clap(short = 'm', long, value_parser, default_value_t = false)]
    pub man_page: bool,

    /// Copy files instead of moving them
    #[clap(short, long, value_parser, default_value_t = false)]
    pub copy: bool,

    /// Add this extension
    #[clap(short, long, value_parser)]
    pub add: Option<String>,

    /// Remove this extension
    #[clap(short, long, value_parser)]
    pub remove: Option<String>,
}

pub fn get_args() -> Args {
    Args::parse()
}
