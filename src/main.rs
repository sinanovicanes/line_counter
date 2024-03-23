mod directory;
pub mod file;
use dialoguer::MultiSelect;
use directory::Directory;

use std::{collections::HashSet, env, path::PathBuf};

fn get_path_from_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return None;
    }

    return Some(args[1].clone());
}

fn get_file_extensions_to_show<'a>(founded_extensions: &'a Vec<&'a String>) -> HashSet<&'a String> {
    let selection = MultiSelect::new()
        .with_prompt("Filter files")
        .items(founded_extensions)
        .interact()
        .unwrap_or(Vec::new());

    HashSet::from_iter(selection.into_iter().map(|idx| founded_extensions[idx]))
}

fn main() -> Result<(), &'static str> {
    let path = PathBuf::from(get_path_from_args().expect("Invalid path"));
    let directory = Directory::build(&path)?;
    let total_count = directory.get_line_count();
    let extensions = directory.get_file_extensions();
    let extensions = get_file_extensions_to_show(&extensions);
    let filtered_count = directory.get_line_count_with_extensions(&extensions);

    println!("Total number of lines: {total_count}");
    println!("Filtered number of lines: {filtered_count}");

    Ok(())
}
