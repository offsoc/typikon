use reqwest::blocking::get;
use std::fs::{self, File};
use std::io::{self, copy};
use std::path::Path;
use walkdir::WalkDir;
use zip::read::ZipArchive;

// Recursive function to copy directory contents
pub fn copy_file(from: &Path, to: &Path) -> io::Result<()> {
    // Create target directory if it doesn't exist
    if !to.exists() {
        fs::create_dir_all(to)?;
    }

    // Iterate over entries in source directory
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(from).unwrap();
        let target_path = to.join(relative_path);

        if path.is_dir() {
            // Recursively copy sub-directory
            copy_file(&path, &target_path)?;
        } else if path.is_file() {
            // Copy file
            fs::copy(&path, &target_path)?;
        }
    }

    Ok(())
}

pub fn remove_md_extension(filename: &str) -> String {
    if let Some(idx) = filename.rfind('.') {
        let (name, ext) = filename.split_at(idx);
        if ext.to_lowercase() == ".md" {
            return name.to_string();
        }
    }
    filename.to_string()
}

pub fn download_zip() -> Result<(), Box<dyn std::error::Error>> {
    let repo_url = "https://github.com/typikonbook/typikon-book/archive/refs/heads/main.zip";
    let zip_path = Path::new("repo.zip");
    let extract_path = Path::new(".");

    let mut response = get(repo_url)?;
    let mut out = File::create(zip_path)?;
    copy(&mut response, &mut out)?;
    let zip_file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(zip_file)?;
    archive.extract(extract_path)?;

    Ok(())
}

pub fn move_dir_contents(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in WalkDir::new(src).min_depth(1) {
        let entry = entry?;
        let entry_path = entry.path();
        let relative_path = entry_path
            .strip_prefix(src)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let dest_path = dst.join(relative_path);

        if entry_path.is_dir() {
            fs::create_dir_all(&dest_path)?;
        } else {
            fs::rename(entry_path, &dest_path)?;
        }
    }

    Ok(())
}

pub fn delete_file(file_path: &Path) -> io::Result<()> {
    if file_path.exists() && file_path.is_file() {
        fs::remove_file(file_path)?;
    }
    Ok(())
}

pub fn delete_folder(folder_path: &Path) -> io::Result<()> {
    if folder_path.exists() && folder_path.is_dir() {
        fs::remove_dir_all(folder_path)?;
    }
    Ok(())
}
