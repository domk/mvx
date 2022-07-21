use crate::arguments::Args;
use man::prelude::*;
use regex::Regex;
use std::error::Error;


pub fn get_man() -> String {
    Manual::new("mvx")
        .about("Move extension")
        .author(Author::new("Dominik Madon").email("dominik@acm.org"))
        .flag(
            Flag::new()
                .short("-h")
                .long("--help")
                .help("Print the help page"),
        )
        .flag(
            Flag::new()
                .short("-n")
                .long("--dry-run")
                .help("Dry run mode"),
        )
        .flag(
            Flag::new()
                .short("-c")
                .long("--copy")
                .help("copy mode (instead of move)"),
        )
        .option(
            Opt::new("add")
                .short("-a")
                .long("--add")
                .help("The extension to be added (including the dot)"),
        )
        .option(
            Opt::new("remove")
                .short("-r")
                .long("--remove")
                .help("The extension to be removed (including the dot)"),
        )
        .example(
            Example::new()
                .text(
                    "Rename all extension files *.bak from '.bak' files to '~'"
                )
                .command(r#"mvx -a "~" -r .bak *.bak"#)
            )
        .custom(
            Section::new("usage note")
                .paragraph("When the program is called cpx the --copy flag is automatically set.")
        )
        .render()
}

/// Return the file's base name if the given extension is removed
/// including possible dot.
///
/// ```
/// # use mvx::utils::get_basename;
/// assert_eq!("foo", get_basename("foo.bar", ".bar").unwrap());
/// ```

pub fn get_basename<'a, 'b>(
    filename: &'b str,
    ext: &'a str
) -> Option<&'b str> {
    let re: Regex = Regex::new(&format!("(.*){}", ext)).unwrap();

    if re.is_match(filename) {
        let capture = re.captures(filename).unwrap();
        Some(capture.get(1).map_or("", |m| m.as_str()))
    } else {
        None
    }
}

/// Check if files to be processed exist.
pub fn check_files(args: &Args) -> Result<(), Box<dyn Error>> {
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
