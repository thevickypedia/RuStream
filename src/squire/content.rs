use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::{Deserialize, Serialize};

/// Represents the payload structure for content, including files and directories.
///
/// This struct is used for serialization and deserialization, providing default values
/// when necessary.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContentPayload {
    /// List of files with their names and paths.
    #[serde(default = "default_structure")]
    pub files: Vec<HashMap<String, String>>,
    /// List of directories with their names and paths.
    #[serde(default = "default_structure")]
    pub directories: Vec<HashMap<String, String>>,
}

/// Returns the default structure for content, represented as an empty vector of HashMaps.
pub fn default_structure() -> Vec<HashMap<String, String>> {
    Vec::new()
}

/// Extracts a natural sort key from a filename.
///
/// This function takes a filename as input and splits it into a list of parts using a regular expression.
/// It then converts numeric parts to integers while keeping non-numeric parts as lowercase strings.
/// This enables a natural sorting order that considers both alphabetical and numerical components of filenames,
/// making it suitable for sorting filenames in a human-friendly manner.
///
/// # Arguments
///
/// * `filename` - A string representing the filename.
///
/// # Returns
///
/// A vector of `Result<i32, String>` where each element is either an integer representing a numeric part
/// or a string representing a non-numeric part converted to lowercase.
fn natural_sort_key(filename: &str) -> Vec<std::result::Result<i32, String>> {
    let re = Regex::new(r"(\D+|\d+)").unwrap();
    re.find_iter(filename)
        .map(|part| {
            if let Ok(num) = part.as_str().parse::<i32>() {
                Ok(num)
            } else {
                Err(part.as_str().to_string())
            }
        })
        .collect()
}

/// Retrieves content information for a specific directory within a stream.
///
/// # Arguments
///
/// * `args` - A tuple containing a stream identifier, a directory path, and references to two strings.
///
/// # Returns
///
/// A `ContentPayload` struct representing the content of the specified directory.
pub fn get_dir_stream_content(parent: &str, subdir: &str, file_formats: &[String]) -> ContentPayload {
    let mut files = Vec::new();
    for entry in fs::read_dir(parent).unwrap().flatten() {
        let file_name = entry.file_name().into_string().unwrap();
        if file_name.starts_with('_') || file_name.starts_with('.') {
            continue;
        }
        let file_path = Path::new(subdir).join(&file_name);
        let file_extn = &file_path.extension().unwrap().to_string_lossy().to_string();
        if file_formats.contains(file_extn) {
            let map = HashMap::from([
                ("name".to_string(), file_name),
                ("path".to_string(), file_path.to_string_lossy().to_string())
            ]);
            files.push(map);
        }
    }
    files.sort_by_key(|a| natural_sort_key(a.get("name").unwrap()));
    ContentPayload { files, ..Default::default() }
}

/// Represents an iterator structure with optional previous and next elements.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Iter {
    /// Optional previous element in the iteration.
    pub previous: Option<String>,
    /// Optional next element in the iteration.
    pub next: Option<String>,
}

/// Retrieves iterator information from Python based on the provided arguments.
///
/// # Arguments
///
/// * `args` - A tuple containing a stream identifier and references to two strings.
///
/// # Returns
///
/// An `Iter` struct representing the iterator information.
pub fn get_iter(filepath: &PathBuf, file_formats: &[String]) -> Iter {
    let parent = filepath.parent().unwrap();
    let mut dir_content: Vec<String> = fs::read_dir(parent)
        .ok().unwrap()
        .flatten()
        .filter_map(|entry| {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let file_extn = Path::new(&file_name).extension().unwrap_or_default().to_string_lossy().to_string();
            if file_formats.contains(&file_extn) {
                Some(file_name)
            } else {
                None
            }
        })
        .collect();
    dir_content.sort_by_key(|a| natural_sort_key(a));

    let idx = dir_content.iter().position(|file| file == filepath.file_name().unwrap().to_str().unwrap()).unwrap();

    let previous_ = if idx > 0 {
        let previous_ = &dir_content[idx - 1];
        if previous_ == filepath.file_name().unwrap().to_str().unwrap() {
            None
        } else {
            Some(previous_.clone())
        }
    } else {
        None
    };

    let next_ = dir_content.get(idx + 1).cloned();

    Iter { previous: previous_, next: next_ }
}
