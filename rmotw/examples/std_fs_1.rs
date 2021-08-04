// std::fs (Part 1)
// Author: Rudgal The Delirious

use std::{fs, io};
use std::path::{Path, PathBuf};

// You should probably change this to an appropriate directory for you :)
const PHOTO_HOME: &str = "/home/rudgal/birds";

/// Recursively visits a path, returning a flattened list of ALL items
/// at or below the path
fn visit_dirs(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = vec![];
    if dir.is_dir() {
        let mut dir_files = vec![];
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.push(visit_dirs(&path)?);
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

    // Recursively get files in a directory
    let all_files = visit_dirs(&PathBuf::from(PHOTO_HOME))?;
    println!("The number of files (recursively is {}", all_files.len());

    Ok(())
}
