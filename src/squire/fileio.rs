use std::collections::HashMap;

use pyo3::{Py, PyAny, PyResult, Python};
use pyo3::prelude::PyModule;
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

/// Converts a JSON-formatted string into a `ContentPayload` struct.
///
/// # Arguments
///
/// * `content` - A JSON-formatted string containing content information.
///
/// # Returns
///
/// A `ContentPayload` struct representing the deserialized content.
pub fn convert_to_json(content: String) -> ContentPayload {
    let output: serde_json::Result<ContentPayload> = serde_json::from_str(&content);
    match output {
        Ok(raw_config) => raw_config,
        Err(err) => {
            log::error!("Error deserializing JSON: {}", err);
            log::error!("Raw content from Python: {:?}", content);
            ContentPayload::default()
        }
    }
}

/// Retrieves content information for all streams.
///
/// # Arguments
///
/// * `args` - A tuple containing a stream identifier, and references to two strings.
///
/// # Returns
///
/// A `ContentPayload` struct representing the content of all streams.
pub fn get_all_stream_content(args: (String, (&String, &String))) -> ContentPayload {
    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/python/fileio.py"));
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let app: Py<PyAny> = PyModule::from_code(py, py_app, "", "")?
            .getattr("get_all_stream_content")?
            .into();
        app.call1(py, args)
    });
    convert_to_json(from_python.unwrap().to_string())
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
pub fn get_dir_stream_content(args: (String, String, (&String, &String))) -> ContentPayload {
    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/python/fileio.py"));
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let app: Py<PyAny> = PyModule::from_code(py, py_app, "", "")?
            .getattr("get_dir_stream_content")?
            .into();
        app.call1(py, args)
    });
    convert_to_json(from_python.unwrap().to_string())
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
pub fn get_iter(args: (&String, (&String, &String))) -> Iter {
    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/python/fileio.py"));
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let app: Py<PyAny> = PyModule::from_code(py, py_app, "", "")?
            .getattr("get_iter")?
            .into();
        app.call1(py, args)
    });
    match from_python {
        Ok(result) => {
            let content = result.to_string();
            let output: serde_json::Result<Iter> = serde_json::from_str(&content);
            match output {
                Ok(parsed_vector) => parsed_vector,
                Err(err) => {
                    log::error!("Error parsing JSON response from Python: {}", err);
                    log::error!("Raw content from Python: {}", content);
                    Iter::default()
                }
            }
        }
        Err(err) => {
            log::error!("Error calling Python function: {}", err);
            Iter::default()
        }
    }
}

/// Converts an SRT file to VTT format.
///
/// # Arguments
///
/// * `input_file` - The path to the input SRT file.
///
/// # Returns
///
/// A boolean indicating whether the conversion was successful.
pub fn srt_to_vtt(input_file: &String) -> bool {
    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/python/fileio.py"));
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let app: Py<PyAny> = PyModule::from_code(py, py_app, "", "")?
            .getattr("srt_to_vtt")?
            .into();
        app.call1(py, (input_file, ))
    });
    match from_python {
        Ok(result) => {
            let result_string = result.to_string();
            match result_string.as_str() {
                "true" => true,
                "false" => false,
                _ => false,
            }
        }
        Err(err) => {
            log::error!("{}", err);
            false
        }
    }
}
