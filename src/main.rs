// use clap::{App, Arg, ArgMatches};
use clap::Parser;
use regex::Regex;
use std::error::Error;
use std::fs;
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

/// Type `Operation` represents a function able to process a file
/// wathever the operation.
type Operation<'a> =
    &'a dyn Fn(&Args, &str, &str) -> Result<(), Box<dyn Error>>;

/// Copy a file or just show what will happen if `args.copy` is true.
fn copy_fn(
    args: &Args,
    source: &str,
    target: &str,
) -> Result<(), Box<dyn Error>> {
    if args.dry_run {
        println!("cp {} {}", source, target);
    } else {
        fs::copy(source, target)?;
    }
    Ok(())
}

/// Move a file or just show what will happen if `args.copy` is true.
fn mv_fn(
    args: &Args,
    source: &str,
    target: &str,
) -> Result<(), Box<dyn Error>> {
    if args.dry_run {
        println!("mv {} {}", source, target);
    } else {
        fs::rename(source, target)?;
    }
    Ok(())
}

/// Return the file's base name if the given extension is removed
/// including possible dot.
///
/// ```
/// # use crate::get_basename;
/// assert_eq!("foo", get_basename("foo.bar", ".bar"));
/// ```

fn get_basename<'a, 'b>(filename: &'b str, ext: &'a str) -> Option<&'b str> {
    let re: Regex = Regex::new(&format!("(.*){}", ext)).unwrap();

    if re.is_match(filename) {
        let capture = re.captures(filename).unwrap();
        Some(capture.get(1).map_or("", |m| m.as_str()))
    } else {
        None
    }
}

/// Check if files to be processed exist.
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

/// Process files in order given driven by operation function.
fn process(args: &Args, operation: Operation) -> Result<(), Box<dyn Error>> {
    for filename in &args.filenames {
        let source = filename.to_str().unwrap();
        let mut target: String;

        // Determine what is the target name.
        if let Some(ext_remove) = &args.remove {
            target = get_basename(&source, ext_remove).unwrap().into();
            if let Some(ext_add) = &args.add {
                target = format!("{}{}", target, ext_add);
            }
        } else {
            if let Some(ext_add) = &args.add {
                target = format!("{}{}", source, ext_add);
            } else {
                return Err(Box::from(
                    "One must provide either --remove or --add command",
                ));
            }
        }

        (operation)(args, source, &target)?;
    }

    Ok(())
}

/// Set context by preparing the function (op) to be applied in
/// process().
fn operation(args: &Args) -> Operation {
    if args.copy {
        &copy_fn
    } else {
        &mv_fn
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    if args.filenames.len() == 0 {
        return Err(Box::from("At least one argument must be provided"));
    }

    check_files(&args)?;
    process(&args, &operation(&args))?;

    Ok(())
}
