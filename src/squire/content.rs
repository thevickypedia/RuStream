use std::collections::HashMap;
use std::fs;
use std::path::Path;

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
pub fn get_dir_stream_content(parent: &str, subdir: &str, file_formats: &Vec<String>) -> ContentPayload {
    let mut files = Vec::new();
    for entry in fs::read_dir(parent).unwrap() {
        if let Ok(entry) = entry {
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
    }
    files.sort_by_key(|a| natural_sort_key(a.get("name").unwrap()));
    let mut content_payload = ContentPayload::default();
    content_payload.files = files;
    content_payload
}
