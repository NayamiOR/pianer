/*
 * Copyright 2025. NayamiOR
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */

use std::path::PathBuf;
use walkdir::WalkDir;

pub fn load_files(path: &String, recursive: bool) -> Result<Vec<PathBuf>, String> {
    if recursive {
        collect_paths_recursive(path)
    } else {
        collect_paths_non_recursive(path)
    }
}

fn collect_paths_recursive(path: &str) -> Result<Vec<PathBuf>, String> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| Ok(e.into_path()))
        .collect()
}

fn collect_paths_non_recursive(path: &str) -> Result<Vec<PathBuf>, String> {
    WalkDir::new(path)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| Ok(e.into_path()))
        .collect()
}

pub fn check_valid_path(path: &String) -> bool {
    let path = std::path::Path::new(path);
    if path.exists() {
        return true;
    }
    false
}

pub fn open_folder(path: String) -> Result<(), std::io::Error> {
    let path = std::path::Path::new(&path);
    if path.exists() {
        std::process::Command::new("explorer").arg(path).spawn()?;
    }
    Ok(())
}
