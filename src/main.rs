use mvx::utils::{check_files, get_man};
use mvx::arguments::get_args;
use mvx::command::{operation, process};
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let args = get_args();

    if args.man_page {
        println!("{}", get_man());
        return Ok(())
    }

    if args.filenames.len() == 0 {
        return Err(Box::from("At least one filename  must be provided"))
    }

    check_files(&args)?;
    process(&args, &operation(&args))?;

    Ok(())
}
