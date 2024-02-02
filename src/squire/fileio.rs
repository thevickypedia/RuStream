use std::collections::HashMap;
use std::path;

use walkdir::WalkDir;

struct FileSystem {
    files: Vec<String>,
    directories: Vec<String>,
}

fn natural_sort_key(filename: &str) -> Vec<Result<i32, String>> {
    let parts: Vec<_> = filename.split_terminator(char::is_numeric).map(|s| s.to_string()).collect();
    parts.into_iter().map(|part| part.parse::<i32>().map_err(|_| part)).collect()
}

pub fn get_all_stream_content(video_source: &str, file_formats: Vec<String>) -> FileSystem {
    let mut structure = FileSystem {
        files: Vec::new(),
        directories: Vec::new(),
    };

    for entry in WalkDir::new(video_source)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();
        let filename = entry_path.file_name();
        let file_extn = entry_path.extension();
        if filename.is_some() && file_extn.is_some() {
            let fname = filename.unwrap().to_string_lossy();
            if fname.starts_with("_") || fname.starts_with(".") { continue; }
            let extn = format!(".{}", file_extn.unwrap().to_string_lossy());
            if file_formats.contains(&extn) {
                let new_path = entry_path.to_string_lossy()
                    .replace(video_source, "")
                    .replace(&fname.to_string(), "")
                    .strip_prefix("/").unwrap().to_string();
                if new_path.is_empty() {
                    structure.files.push(fname.to_string())
                } else {
                    let mut entry: HashMap<String, String> = HashMap::new();
                    entry.insert("name".to_string(), new_path);
                    let key = path::Path::new("stream").join(new_path.to_string());
                    entry.insert("path".to_string(), key.to_str().unwrap().to_string());
                    if structure.directories.contains(entry) {
                        continue
                    }
                }
                println!("{} - {}", new_path, fname);
            }
        }
    }
    for (_, entries) in structure.iter_mut() {
        entries.sort_by_key(|entry| natural_sort_key(&entry["name"]));
    }
    structure
}
