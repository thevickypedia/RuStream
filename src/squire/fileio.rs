use std::collections::HashMap;
use std::path;

use pyo3::{Py, PyAny, PyResult, Python};
use pyo3::prelude::PyModule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContentPayload {
    pub files: Vec<HashMap<String, String>>,
    #[serde(default = "default_structure")]
    pub directories: Vec<HashMap<String, String>>,
}

pub fn default_structure() -> Vec<HashMap<String, String>> { Vec::new() }

fn delete_file(file_path: String) {
    match std::fs::remove_file(&file_path) {
        Ok(()) => {
            log::debug!("File '{}' successfully deleted.", file_path);
        }
        Err(err) => match err.kind() {
            // Handle specific errors, if necessary
            std::io::ErrorKind::NotFound => {
                log::error!("File '{}' not found.", file_path);
            }
            _ => {
                log::error!("Error deleting file '{}': {}", file_path, err);
            }
        },
    }
}

fn convert_to_json(filename: String) -> ContentPayload {
    let payload;
    if path::Path::new(&filename).exists() {
        match std::fs::read_to_string(&filename) {
            Ok(content) => {
                let output: serde_json::Result<ContentPayload> = serde_json::from_str(&content);
                match output {
                    Ok(raw_config) => {
                        delete_file(filename);
                        payload = raw_config;
                    }
                    Err(err) => {
                        println!("{:?}", content);
                        panic!("Error deserializing JSON: {}", err);
                    }
                }
            }
            Err(err) => {
                panic!("Error reading file: {}", err);
            }
        }
    } else {
        panic!("{} not found", filename)
    }
    payload
}

pub fn get_all_stream_content(args: (String, (&String, &String))) -> ContentPayload {
    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python/fileio.py"));
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let app: Py<PyAny> = PyModule::from_code(py, py_app, "", "")?
            .getattr("get_all_stream_content")?
            .into();
        app.call1(py, args)
    });
    convert_to_json(from_python.unwrap().to_string())
}

pub fn get_dir_stream_content(args: (String, String, (&String, &String))) -> ContentPayload {
    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python/fileio.py"));
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let app: Py<PyAny> = PyModule::from_code(py, py_app, "", "")?
            .getattr("get_dir_stream_content")?
            .into();
        app.call1(py, args)
    });
    convert_to_json(from_python.unwrap().to_string())
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Iter {
    pub previous: Option<String>,
    pub next: Option<String>,
}

pub fn get_iter(args: (&String, (&String, &String))) -> Iter {
    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python/fileio.py"));
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let app: Py<PyAny> = PyModule::from_code(py, py_app, "", "")?
            .getattr("get_iter")?
            .into();
        app.call1(py, args)
    });
    let content = from_python.unwrap().to_string();
    let output: serde_json::Result<Iter> = serde_json::from_str(&content);
    match output {
        Ok(parsed_vector) => {
            return parsed_vector;
        }
        Err(e) => {
            log::error!("Error parsing JSON: {}", e);
        }
    }
    let previous = None;
    let next = None;
    Iter { previous, next }
}
