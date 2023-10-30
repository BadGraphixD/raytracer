extern crate walkdir;

use std::{
    env,
    fs::{self, DirBuilder},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let source_res_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("res");

    let target_res_path = locate_target_dir_from_output_dir(&out_dir)
        .expect("Failed to find target dir")
        .join(env::var("PROFILE").unwrap())
        .join("res");

    if target_res_path.exists() {
        fs::remove_dir_all(&target_res_path)
            .expect("Failed to remove preexisting target resource directory");
    }

    copy(&source_res_path, &target_res_path);
}

fn locate_target_dir_from_output_dir(mut target_dir_search: &Path) -> Option<&Path> {
    loop {
        if target_dir_search.ends_with("target") {
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
