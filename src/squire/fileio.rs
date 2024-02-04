use std::collections::HashMap;

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

fn convert_to_json(content: String) -> ContentPayload {
    let output: serde_json::Result<ContentPayload> = serde_json::from_str(&content);
    let payload = match output {
        Ok(raw_config) => {
            raw_config
        }
        Err(err) => {
            println!("{:?}", content);
            panic!("Error deserializing JSON: {}", err);
        }
    };
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
