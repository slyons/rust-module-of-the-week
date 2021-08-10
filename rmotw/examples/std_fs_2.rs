use std::time::SystemTime;
use std::{fs, io};
use std::path::{Path, PathBuf};
use std::collections::{HashSet, BTreeMap};

// Re-using from part 1
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

const YEAR_SECONDS: u64 = 31556926;
const MONTH_SECONDS: u64 = 2629743;
const DAY_SECONDS: u64 = 86400;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct RudgalDate {
    year: u64,
    month: u64,
    day: u64
}

fn systime_to_tuple(t: SystemTime) -> RudgalDate {
    // This is an approximate, do not use in any sort of production
    let secs = t.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let years = secs / YEAR_SECONDS;
    let months = (secs % YEAR_SECONDS) / MONTH_SECONDS;
    let days = ((secs % YEAR_SECONDS) % MONTH_SECONDS) / DAY_SECONDS;
    RudgalDate {
        year:1970+years, 
        month: 1+months, 
        day: 1+days
    }
}

// Group by year, month, day
fn group_files_by_date(paths: Vec<PathBuf>) -> 
    io::Result<BTreeMap<RudgalDate, HashSet<PathBuf>>>
{
    let mut result = BTreeMap::new();
    paths.clone().into_iter()
        .for_each(|p| {
            let modified = fs::metadata(&p).and_then(|m| m.modified()).unwrap();
            let map = result.entry(systime_to_tuple(modified))
                .or_insert(HashSet::new());
            map.insert(p);
        });
    Ok(result)
}

fn link_files(link_root: &Path, files: &BTreeMap<RudgalDate, HashSet<PathBuf>>)
    -> io::Result<()>
{
    if link_root.exists() {
        println!("Removing directory {:?} and everything below it", link_root);
        fs::remove_dir_all(&link_root)?; 
    }

    let mut link_root = link_root.to_path_buf();
    for (date, date_files) in files {
        link_root.push(format!("{}/{}/{}", date.year, date.month, date.day));
        for file in date_files {
            // Ensure the destination exists already!
            fs::create_dir_all(&link_root)?;
            link_root.push(file.file_name().and_then(|f|f.to_str()).unwrap());
            fs::hard_link(file, &link_root)?;
            link_root.pop();
        }
        link_root.pop();
        link_root.pop();
        link_root.pop();
    }

    Ok(())
}

// Copy files to new directories
fn copy_files(copy_root: &Path, files: &BTreeMap<RudgalDate, HashSet<PathBuf>>)
    -> io::Result<()>
{
    if copy_root.exists() {
        println!("Removing directory {:?} and everything below it", copy_root);
        fs::remove_dir_all(&copy_root)?;
    }

    let mut copy_root = copy_root.to_path_buf();
    for (date, date_files) in files {
        copy_root.push(format!("{}/{}/{}", date.year, date.month, date.day));
        for file in date_files {
            // Ensure the destination exists already!
            fs::create_dir_all(&copy_root)?;
            copy_root.push(file.file_name().and_then(|f|f.to_str()).unwrap());

            // Copy from the original to the newly-sorted path
            fs::copy(file, &copy_root)?;
            
            copy_root.pop();
        }
        copy_root.pop();
        copy_root.pop();
        copy_root.pop();
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // If you want some example "images" to play along with, use 
    // scripts/generate_files.py <path>
    // where <path> is the place to put the fake images
    let image_source = "./images";
    let image_link_dest = "./images_links";
    let image_dest = "./images_sorted";

    println!("Welcome to Rudgal's Image organizer");

    let image_link_dest = PathBuf::from(image_link_dest);
    let image_dest = PathBuf::from(image_dest);
    
    let files = iter_dirs(&Path::new(image_source))?;
    let files_count = files.len();
    println!("There are {} images!", files_count);

    // Group files by Year, Month and Day
    let grouped = group_files_by_date(files)?;

    let files_per_key = grouped.iter()
        .map(|(k, v)| (k, v.len()))
        .collect::<Vec<_>>();
    println!("Some dates from the files are {:?}...", &files_per_key[0..3]);

    println!("Linking the pictures");
    link_files(&image_link_dest, &grouped)?;
    let link_count = iter_dirs(&image_link_dest)?.len();
    println!("{} images linked", link_count);
    assert_eq!(files_count, link_count);

    println!("Copying the pictures");
    copy_files(&image_dest, &grouped)?;
    let copied_count = iter_dirs(&image_dest)?.len();
    println!("{} images copied", copied_count);
    assert_eq!(files_count, copied_count);

    Ok(())
}