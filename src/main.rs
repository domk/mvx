// use clap::{App, Arg, ArgMatches};
use clap::Parser;
use std::fs;
use regex::Regex;
use std::error::Error;
use std::path::PathBuf;

const SEMVER: &str = "0.1";

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
struct Args {
    /// Files to operate on
    #[clap(value_parser)]
    filenames: Vec<PathBuf>,

    /// Dry run, don't change filesystem
    #[clap(short = 'n', long, value_parser, default_value_t = false)]
    dry_run: bool,

    /// Copy files instead of moving them
    #[clap(short, long, value_parser, default_value_t = false)]
    copy: bool,

    /// Add this extension
    #[clap(short, long, value_parser)]
    add: Option<String>,

    /// Remove this extension
    #[clap(short, long, value_parser)]
    remove: Option<String>,
}

fn get_basename<'a, 'b>(filename: &'b str, ext: &'a str) -> Option<&'b str> {
    let re: Regex = Regex::new(&format!("(.*){}", ext)).unwrap();

    if re.is_match(filename) {
        let capture = re.captures(filename).unwrap();
        Some(capture.get(1).map_or("", |m| m.as_str()))
    } else {
        None
    }
}

fn check_files(args: &Args) -> Result<(), Box<dyn Error>> {
    for filename in &args.filenames {
        if filename.as_path().exists() {
            if let Some(extension) = &args.remove {
                let fname = filename.to_str().unwrap();
                if get_basename(fname, extension) == None {
                    return Err(Box::from(format!(
                        "{} does not end with {}",
                        fname, extension
                    )));
                }
            }
        } else {
            return Err(Box::from(format!(
                "{} does not exist.",
                filename.display()
            )));
        }
    }

    Ok(())
}

fn process_dry_run(args: &Args) -> Result<(), Box<dyn Error>> {
    let command = if args.copy { "cp" } else { "mv" };
    for filename in &args.filenames {
        let fname = filename.to_str().unwrap();
        if let Some(ext_remove) = &args.remove {
            let basename = get_basename(fname, ext_remove).unwrap();
            if let Some(ext_add) = &args.add {
                let target = format!("{}{}", basename, ext_add);
                println!("{} {} {}", command, fname, target);
            } else {
                println!("{} {} {}", command, fname, basename);
            }
        } else {
            if let Some(ext_add) = &args.add {
                let target = format!("{}{}", fname, ext_add);
                println!("{} {} {}", command, fname, target);
            } else {
                return Err(Box::from(
                    "One must provide either --remove or --add command",
                ));
            }
        }
    }

    Ok(())
}

fn process(args: &Args) -> Result<(), Box<dyn Error>> {
    for filename in &args.filenames {
        let fname = filename.to_str().unwrap();
        if let Some(ext_remove) = &args.remove {
            let basename = get_basename(fname, ext_remove).unwrap();
            if let Some(ext_add) = &args.add {
                let target = format!("{}{}", basename, ext_add);
		if args.copy {
		    fs::copy(fname, target)?;
		} else {
		    fs::rename(fname, target)?;
		}
            } else {
		if args.copy {
		    fs::copy(fname, basename)?;
		} else {
		    fs::rename(fname, basename)?;
		}
            }
        } else {
            if let Some(ext_add) = &args.add {
                let target = format!("{}{}", fname, ext_add);
		if args.copy {
		    fs::copy(fname, target)?;
		} else {
		    fs::rename(fname, target)?;
		}
            } else {
                return Err(Box::from(
                    "One must provide either --remove or --add command",
                ));
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    check_files(&args)?;
    if args.dry_run {
	process_dry_run(&args)?;
    } else {
	process(&args)?;
    }

    Ok(())
}
