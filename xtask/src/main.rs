mod code_snippet;
mod daily_challenge;
mod write_file;

use std::error::Error;

use convert_case::{Case, Casing};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    let title_slug = if args.len() == 1 {
        daily_challenge::get_daily_challenge_slug()
    } else if args.len() != 2 {
        return Err("Usage: binary SLUG".into());
    } else {
        args.nth(1).unwrap()
    };
    let slug_snake = title_slug.to_case(Case::Snake);
    let code_snippet = code_snippet::get_code_snippet(&title_slug);
    write_file::write_file(&slug_snake, code_snippet)?;
    Ok(())
}
