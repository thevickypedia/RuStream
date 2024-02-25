use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::squire::settings::Config;

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
fn natural_sort_key(filename: &str) -> Vec<Result<i32, String>> {
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


/// Retrieves content information for all streams.
///
/// # Arguments
///
/// * `config` - Configuration data for the application.
///
/// # Returns
///
/// A `ContentPayload` struct representing the content of all streams.
pub fn get_all_stream_content(config: &Config) -> ContentPayload {
    let mut payload = ContentPayload::default();

    for entry in WalkDir::new(&config.video_source).into_iter().filter_map(|e| e.ok()) {
        if entry.path().ends_with("__") {
            continue;
        }

        if let Some(file_name) = entry.file_name().to_str() {
            if file_name.starts_with('_') || file_name.starts_with('.') {
                continue;
            }

            if let Some(extension) = PathBuf::from(file_name).extension().and_then(|ext| ext.to_str()) {
                if config.file_formats.iter().any(|format| extension == format) {
                    let path = entry.path().strip_prefix(&config.video_source)
                        .unwrap_or_else(|_| Path::new(""));
                    let components: &Vec<_> = &path.components().collect();
                    if components.len() == 1 {
                        let mut entry_map = HashMap::new();
                        entry_map.insert("path".to_string(), format!("stream/{}", &file_name));
                        entry_map.insert("name".to_string(), file_name.to_string());
                        payload.files.push(entry_map);
                    } else {
                        /*
                        path.components(): returns an iterator over the components of the path
                        .rev(): reverses the order of the iterator
                        .skip(1): skips the first (originally last) component of the reversed path
                         */
                        let skimmed: String = path.components().rev().skip(1)
                            .collect::<Vec<_>>().iter().rev()
                            .collect::<PathBuf>().to_string_lossy().to_string();
                        let mut entry_map = HashMap::new();
                        entry_map.insert("path".to_string(), format!("stream/{}", &skimmed));
                        entry_map.insert("name".to_string(), skimmed);
                        if payload.directories.contains(&entry_map) { continue; }
                        payload.directories.push(entry_map);
                    }
                }
            }
        }
    }

    payload.files.sort_by(|a, b| natural_sort_key(&a["name"]).cmp(&natural_sort_key(&b["name"])));
    payload.directories.sort_by(|a, b| natural_sort_key(&a["name"]).cmp(&natural_sort_key(&b["name"])));

    payload
}


/// Retrieves content information for a specific directory within a stream.
///
/// # Arguments
///
/// * `parent` - Path to the parent directory.
/// * `child` - Path to the child directory.
/// * `file_formats` - File formats (set as env vars) that are allowed for streaming.
///
/// # Returns
///
/// A `ContentPayload` struct representing the content of the specified directory.
pub fn get_dir_stream_content(parent: &str, child: &str, file_formats: &[String]) -> ContentPayload {
    let mut files = Vec::new();
    for entry in fs::read_dir(parent).unwrap().flatten() {
        let file_name = entry.file_name().into_string().unwrap();
        if file_name.starts_with('_') || file_name.starts_with('.') {
            continue;
        }
        let file_path = Path::new(child).join(&file_name);
        let file_extn = &file_path.extension().unwrap_or_default().to_string_lossy().to_string();
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

/// Retrieves the previous and/or next file to the currently streaming file.
///
/// # Arguments
///
/// * `filepath` - File that is requested for streaming.
/// * `file_formats` - Vector of file formats (as String) that are allowed.
///
/// # Returns
///
/// An `Iter` struct representing the iterator information.
pub fn get_iter(filepath: &Path, file_formats: &[String]) -> Iter {
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
