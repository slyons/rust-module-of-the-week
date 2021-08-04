// std::fs (Part 1)
// Author: Rudgal The Delirious

use std::{fs, io};
use std::path::{Path, PathBuf};

// You should probably change this to an appropriate directory for you :)
const PHOTO_HOME: &str = "/home/rudgal/birds";

/// Recursively visits a path, returning a flattened list of ALL items
/// at or below the path
/// Iterative version
fn iter_dirs(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut stack = vec![fs::read_dir(dir)?];
    let mut files = vec![];
    // Look out for our future dive into Vectors and their various uses!
    while let Some(dir) = stack.last_mut() {
        // Transpose says: Take that Option<Result> and turn it into a Result<Option>!
        match dir.next().transpose()? {
            None => {
                stack.pop();
            }
            // A Some! But only if it's the kind of Some we want
            Some(dir) if dir.file_type().map_or(false, |t| t.is_dir()) => {
                stack.push(fs::read_dir(dir.path())?);
            }
            Some(file) => files.push(file.path()),
        }
    }
    Ok(files)
}

/// Recursive version
fn recurse_dirs(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = vec![];
    if dir.is_dir() {
        let mut dir_files = vec![];
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.push(recurse_dirs(&path)?);
            } else {
                dir_files.push(path);
            }
        }
        files.push(dir_files);
    }
    Ok(files.into_iter().flatten().collect())
}


fn main() -> io::Result<()> {
    // Listing all of the files in a directory
    let entries = fs::read_dir(PHOTO_HOME)?
        .map(|entry_res| entry_res.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    println!("{:?}", entries);

    // Filtering files in a directory
    let just_photos = entries.iter()
        .filter_map(|entry| Some((entry, fs::metadata(entry).ok()?)))
        .filter(|(_, meta)| meta.is_file())
        .collect::<Vec<_>>();

    println!("There are {} actual files.", just_photos.len());

    // Getting the size of a file
    let photo_size:u64 = just_photos.iter()
        .map(|(_path, meta)| meta.len())
        .sum();

    println!("Total size is {}", photo_size);

    // Iteratively get files in a directory
    let iter_files = iter_dirs(&PathBuf::from("."))?;
    println!("The number of files (iteratively) is {}", iter_files.len());

    // Recursively get files in a directory
    let recurse_files = recurse_dirs(&PathBuf::from("."))?;
    println!("The number of files (recursively) is {}", recurse_files.len());

    Ok(())
}
