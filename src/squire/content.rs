use std::collections::HashMap;
use std::fs;
use std::path::{MAIN_SEPARATOR, Path, PathBuf};

use regex::Regex;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::constant;
use crate::squire::settings;

/// Represents the payload structure for content, including files and directories.
///
/// This struct is used for serialization and deserialization, providing default values
/// when necessary.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContentPayload {
    /// List of files with their names, paths and font icons.
    #[serde(default = "default_structure")]
    pub files: Vec<HashMap<String, String>>,
    /// List of directories with their names, paths and font icons.
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
/// * `regex` - Pre-compiled regex object.
/// * `filename` - A string representing the filename.
///
/// # Returns
///
/// A vector of `Result<i32, String>` where each element is either an integer representing a numeric part
/// or a string representing a non-numeric part converted to lowercase.
fn natural_sort_key(regex: &Regex, filename: &str) -> Vec<Result<i32, String>> {
    // reusing regex is way faster than creating a new object everytime (~8s
    regex.find_iter(filename)
        .map(|part| {
            // chaining methods is kinda faster (~79% faster in terms of ms)
            part.as_str().parse::<i32>().map_err(|e| e.to_string())
            // if let Ok(num) = part.as_str().parse::<i32>() {
            //     Ok(num)
            // } else {
            //     Err(part.as_str().to_string())
            // }
        })
        .collect()
}


/// Generate font awesome icon's value for a given file extension.
///
/// Creates custom icons for `image` files, defaults to `video` icon.
///
/// # Arguments
///
/// * `extn` - File extension.
///
/// # Returns
///
/// A string with the `fa` value based on the file extension.
pub fn get_file_font(extn: &str) -> String {
    let font = if constant::IMAGE_FORMATS.contains(&extn) {
        "fa-regular fa-file-image"
    } else {
        "fa-regular fa-file-video"
    };
    font.to_string()
}

/// Formats a byte count into a human readable string.
///
/// # Arguments
///
/// * `bytes` - Size of the file in bytes.
///
/// # Returns
///
/// A string like `12.5 MB` or `1.2 GB`.
pub fn format_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{} {}", size as u64, UNITS[unit])
    } else {
        format!("{:.2} {}", size, UNITS[unit])
    }
}

/// Formats seconds into a human readable duration string.
///
/// # Arguments
///
/// * `seconds` - Duration of the media file in seconds.
///
/// # Returns
///
/// A string like `1:05:30` or `2:45`.
pub fn format_duration(seconds: f64) -> String {
    let total = seconds.round() as u64;
    let hours = total / 3600;
    let minutes = (total % 3600) / 60;
    let secs = total % 60;
    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{}:{:02}", minutes, secs)
    }
}

/// Returns whether the file extension belongs to a convertible video format.
pub fn is_video(extension: &str) -> bool {
    constant::VIDEO_FORMATS.contains(&extension.to_lowercase().as_str())
}

/// Checks if a file extension is allowed by the configured formats.
///
/// An empty list or a `*` entry disables filtering, making every file eligible.
fn is_allowed_format(file_formats: &[String], extension: &str) -> bool {
    if file_formats.is_empty() || file_formats.iter().any(|format| format == "*") {
        return true;
    }
    file_formats.iter().any(|format| extension == format)
}

/// Builds the size and duration values for a media file.
///
/// Duration is probed on-demand only for video files and only when ffmpeg
/// support is enabled, otherwise it stays empty.
///
/// # Arguments
///
/// * `server_path` - Path of the file on the server.
/// * `ffmpeg_enabled` - Whether on-demand ffmpeg conversion is allowed.
///
/// # Returns
///
/// A tuple of `(size, duration)` as formatted strings.
fn file_details(server_path: &Path, ffmpeg_enabled: bool) -> (String, String) {
    let size = match std::fs::metadata(server_path) {
        Ok(metadata) => format_size(metadata.len()),
        Err(_) => String::new(),
    };
    let extension = server_path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
        .to_lowercase();
    let mut duration = String::new();
    if ffmpeg_enabled && is_video(&extension) {
        if let Some(seconds) = crate::squire::ffmpeg::get_duration(server_path) {
            duration = format_duration(seconds);
        }
    }
    (size, duration)
}

/// Generate font awesome icon's value for a given folder depth.
///
/// Creates custom icons for `folder-tree`, defaults to `folder` icon.
///
/// # Arguments
///
/// * `tree` - Depth of directories.
///
/// # Returns
///
/// A string with the `fa` value based on the folder depth.
fn get_folder_font(path: &Path, parent: String) -> HashMap<String, String> {
    let mut entry_map = HashMap::new();
    entry_map.insert("path".to_string(), format!("stream/{}", parent));
    entry_map.insert("name".to_string(), parent);
    if path.components().collect::<Vec<_>>().len() - 1 > 1 { // -1 for the file in path
        entry_map.insert("font".to_string(), "fa-solid fa-folder-tree".to_string());
    } else {
        entry_map.insert("font".to_string(), "fa fa-folder".to_string());
    }
    entry_map
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
pub fn get_all_stream_content(config: &settings::Config) -> ContentPayload {
    let mut payload = ContentPayload::default();
    let mut redundant = Vec::new();

    for entry in WalkDir::new(&config.media_source).into_iter().filter_map(|e| e.ok()) {
        if entry.path().ends_with("__") {
            continue;
        }

        if let Some(file_name) = entry.file_name().to_str() {
            if file_name.starts_with('_') || file_name.starts_with('.') {
                continue;
            }

            if let Some(extension) = PathBuf::from(file_name).extension().and_then(|ext| ext.to_str()) {
                if is_allowed_format(&config.file_formats, extension) {
                    let path = entry.path().strip_prefix(&config.media_source)
                        .unwrap_or_else(|_| Path::new(""));
                    let components: &Vec<_> = &path.components().collect();
                    if components.len() == 1 {
                        let mut entry_map = HashMap::new();
                        entry_map.insert("path".to_string(), format!("stream/{}", file_name));
                        entry_map.insert("name".to_string(), file_name.to_string());
                        entry_map.insert("font".to_string(), get_file_font(extension));
                        entry_map.insert("video".to_string(), is_video(extension).to_string());
                        let (size, duration) = file_details(entry.path(), config.ffmpeg_enabled);
                        entry_map.insert("size".to_string(), size);
                        entry_map.insert("duration".to_string(), duration);
                        payload.files.push(entry_map);
                    } else {
                        let parent = path.components().collect::<Vec<_>>()
                            .first().unwrap().as_os_str()
                            .to_string_lossy().to_string();
                        if redundant.contains(&parent) {
                            // skip if at least one file is present in any of the subdirectories
                            continue
                        }
                        redundant.push(parent.clone());
                        let entry_map = get_folder_font(path, parent);
                        if payload.directories.contains(&entry_map) || entry_map.is_empty() { continue; }
                        payload.directories.push(entry_map);
                    }
                }
            }
        }
    }

    let re = Regex::new(r"(\D+|\d+)").unwrap();
    payload.files.sort_by(|a, b| natural_sort_key(&re, &a["name"]).cmp(&natural_sort_key(&re, &b["name"])));
    payload.directories.sort_by(|a, b| natural_sort_key(&re, &a["name"]).cmp(&natural_sort_key(&re, &b["name"])));

    payload
}

/// Retrieves content information for a specific directory within a stream.
///
/// # Arguments
///
/// * `path_payload` - Media path received in the payload.
/// * `parent` - Path to the parent directory.
/// * `file_formats` - File formats (set as env vars) that are allowed for streaming.
///
/// # Returns
///
/// A `ContentPayload` struct representing the content of the specified directory.
pub fn get_dir_stream_content(path_payload: &String,
                              parent: &str,
                              file_formats: &[String],
                              ffmpeg_enabled: bool) -> ContentPayload {
    // todo: subdirectories are not checked for media files, perhaps this is a bad idea
    let mut files = Vec::new();
    let mut directories = Vec::new();
    for entry in fs::read_dir(parent).unwrap().flatten() {
        let entry_name = entry.file_name().into_string().unwrap();
        if entry_name.starts_with('_') || entry_name.starts_with('.') {
            continue;
        }
        // Use server path to verify and client path to communicate back to client
        let server_path = Path::new(parent).join(&entry_name);
        // Use only the final dir in the path, since rest of it will be loaded in the URL itself
        // Not doing this will result in redundant path, like /home/GOT/season1/season1/episode1.mp4 resulting in 404
        let client_path = Path::new(path_payload.split(MAIN_SEPARATOR)
            .next_back().unwrap()).join(&entry_name)
            .to_string_lossy().to_string();
        if server_path.is_file() {
            let file_extn = &server_path.extension().unwrap_or_default().to_string_lossy().to_string();
            if is_allowed_format(file_formats, file_extn) {
                let (size, duration) = file_details(&server_path, ffmpeg_enabled);
                let map = HashMap::from([
                    ("name".to_string(), entry_name),
                    ("path".to_string(), client_path),
                    ("font".to_string(), get_file_font(file_extn)),
                    ("video".to_string(), is_video(file_extn).to_string()),
                    ("size".to_string(), size),
                    ("duration".to_string(), duration)
                ]);
                files.push(map);
            }
        } else if server_path.is_dir() {
            let map = HashMap::from([
                ("name".to_string(), entry_name),
                ("path".to_string(), client_path),
                ("font".to_string(), "fa-solid fa-folder-tree".to_string())
            ]);
            directories.push(map);
        } else {
            log::error!("Something went horribly wrong");
            log::error!("Parent Dir: {}", parent);
            log::error!("Path Payload: {}", path_payload);
        }
    }
    let re = Regex::new(r"(\D+|\d+)").unwrap();
    files.sort_by_key(|a| natural_sort_key(&re, a.get("name").unwrap()));
    directories.sort_by_key(|a| natural_sort_key(&re, a.get("name").unwrap()));
    ContentPayload { files, directories }
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
            if is_allowed_format(file_formats, &file_extn) {
                Some(file_name)
            } else {
                None
            }
        })
        .collect();
    let re = Regex::new(r"(\D+|\d+)").unwrap();
    dir_content.sort_by_key(|a| natural_sort_key(&re, a));

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
