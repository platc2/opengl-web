extern crate core;
extern crate walkdir;

use std::env;
use std::fs::{self, DirBuilder};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let profile = env::var("PROFILE").unwrap();
    let executable_path = locate_target_dir_from_output_dir(&out_dir, &profile[..])
        .expect("Failed to find target dir");

    copy(
        &manifest_dir.join("assets"),
        &executable_path.join("assets"),
    );
}

fn locate_target_dir_from_output_dir<'a>(mut target_dir_search: &'a Path, profile: &'a str) -> Option<&'a Path> {
    loop {
        if target_dir_search.ends_with(profile) {
            return Some(target_dir_search);
        }

        target_dir_search = match target_dir_search.parent() {
            Some(path) => path,
            None => break,
        }
    }

    None
}

fn copy(from: &Path, to: &Path) {
    let from_path: PathBuf = from.into();
    let to_path: PathBuf = to.into();
    for entry in WalkDir::new(from_path.clone()) {
        let entry = entry.unwrap();

        if let Ok(rel_path) = entry.path().strip_prefix(&from_path) {
            let target_path = to_path.join(rel_path);

            if entry.file_type().is_dir() {
                DirBuilder::new()
                    .recursive(true)
                    .create(target_path)
                    .expect("Failed to create target dir");
            } else {
                fs::copy(entry.path(), &target_path).expect("Failed to copy");
            }
        }
    }
}
