use crate::arguments::Args;
use crate::utils::get_basename;
use std::error::Error;
use std::fs;


/// Type `Operation` represents a function able to process a file
/// wathever the operation.
pub type Operation<'a> =
    &'a dyn Fn(&Args, &str, &str) -> Result<(), Box<dyn Error>>;

/// Copy a file or just show what will happen if `args.copy` is true.
pub fn copy_fn(
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
pub fn mv_fn(
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

/// Process files in order given driven by operation function.
pub fn process(
    args: &Args,
    operation: Operation
) -> Result<(), Box<dyn Error>> {
    for filename in &args.filenames {
        let source = filename.to_str().unwrap();
        let mut target: String;

        // Determine what is the target name.
        if let Some(ext_remove) = &args.remove {
            target = get_basename(&source, ext_remove).unwrap().into();
            if let Some(ext_add) = &args.add {
                target = format!("{}{}", target, ext_add);
            }
        } else if let Some(ext_add) = &args.add {
            target = format!("{}{}", source, ext_add);
        } else {
            return Err(Box::from(
                "One must provide either --remove or --add command",
            ));
        }

        (operation)(args, source, &target)?;
    }

    Ok(())
}

/// Set context by preparing the function (op) to be applied in
/// process().
pub fn operation(args: &Args) -> Operation {
    if args.copy {
        &copy_fn
    } else {
        &mv_fn
    }
}
