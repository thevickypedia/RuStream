use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

use serde::Serialize;

use crate::constant;

/// Reads the duration of a media file in seconds using `ffprobe`.
///
/// This is invoked on-demand per file while rendering the listing page and
/// the process is terminated right after the value is extracted.
///
/// # Arguments
///
/// * `filepath` - Path to the media file.
///
/// # Returns
///
/// Returns `Some(f64)` seconds if the probe succeeded, `None` otherwise.
pub fn get_duration(filepath: &Path) -> Option<f64> {
    let output = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-show_entries", "format=duration",
            "-of", "default=noprint_wrappers=1:nokey=1",
        ])
        .arg(filepath)
        .output()
        .ok()?;
    if !output.status.success() {
        log::warn!("ffprobe failed for {:?}", filepath);
        return None;
    }
    let duration = String::from_utf8_lossy(&output.stdout).trim().to_string();
    match duration.parse::<f64>() {
        Ok(seconds) => Some(seconds),
        Err(_) => {
            log::warn!("ffprobe returned an invalid duration '{}' for {:?}", duration, filepath);
            None
        }
    }
}

/// Validates the target format requested for conversion.
///
/// # Arguments
///
/// * `current_format` - Extension of the file being converted.
/// * `target_format` - Extension requested by the user.
///
/// # Returns
///
/// * `Ok(String)` - Lowercased target format if it is valid.
/// * `Err(String)` - Reason for the rejection.
fn validate_format(current_format: &str, target_format: &str) -> Result<String, String> {
    let target = target_format.trim().to_lowercase();
    if !constant::VIDEO_FORMATS.contains(&target.as_str()) {
        return Err(format!("'{}' is not a supported video format", target_format));
    }
    if target == current_format {
        return Err(format!("'{}' is already in the requested format", current_format));
    }
    Ok(target)
}

/// Runs ffmpeg against the given input and output path with extra arguments.
///
/// # Arguments
///
/// * `filepath` - Path to the input media file.
/// * `output_path` - Path to write the converted file.
/// * `extra_args` - Additional ffmpeg arguments inserted between the input and output.
///
/// # Returns
///
/// * `Ok(())` - If ffmpeg finished successfully.
/// * `Err(String)` - ffmpeg stderr, or a reason when ffmpeg could not be invoked.
fn run_ffmpeg(filepath: &Path, output_path: &Path, extra_args: &[&str]) -> Result<(), String> {
    let output = Command::new("ffmpeg")
        .args(["-hide_banner", "-loglevel", "error", "-y", "-i"])
        .arg(filepath)
        .args(extra_args)
        .arg(output_path)
        .output();
    match output {
        Ok(result) if result.status.success() => Ok(()),
        Ok(result) => {
            let stderr = String::from_utf8_lossy(&result.stderr).trim().to_string();
            if stderr.is_empty() {
                Err(format!("ffmpeg exited with status '{}'", result.status))
            } else {
                Err(stderr)
            }
        }
        Err(error) => Err(format!("Unable to invoke ffmpeg: {}", error)),
    }
}

/// Converts a media file to the requested format using `ffmpeg`.
///
/// A stream copy (remux) is attempted first: when the codecs are already
/// compatible with the target container this skips re-encoding entirely, which
/// is nearly instant and preserves the original quality bit for bit. When the
/// codecs are incompatible a full re-encode is used as fallback, without any
/// quality/ resolution adjustments.
///
/// ffmpeg is only spawned for this request and the process terminates as soon
/// as the conversion completes. The converted file is written next to the
/// original file and the original is NEVER removed.
///
/// # Arguments
///
/// * `filepath` - Path to the media file that has to be converted.
/// * `target_format` - Format the file should be converted into.
///
/// # Returns
///
/// * `Ok(String)` - Path of the converted file on success.
/// * `Err(String)` - Reason for the failure.
pub fn convert(filepath: &Path, target_format: &str) -> Result<String, String> {
    let output_path = validate(filepath, target_format)?;
    let target = output_path.extension().unwrap().to_str().unwrap().to_string();

    log::info!("Converting {:?} to {}", filepath, target);
    match run_ffmpeg(filepath, &output_path, &["-c", "copy"]) {
        Ok(_) => {
            log::info!("Remuxed {:?} to {:?} with stream copy", filepath, output_path);
        }
        Err(reason) => {
            log::debug!("Stream copy failed for {:?}, falling back to re-encode: {}", filepath, reason);
            let _ = std::fs::remove_file(&output_path);
            run_ffmpeg(filepath, &output_path, &["-threads", "0"])
                .map_err(|detail| format!("ffmpeg failed while converting to '{}': {}", target, detail))?;
        }
    }
    log::info!("Converted {:?} to {:?}", filepath, output_path);
    Ok(output_path.to_string_lossy().to_string())
}

/// Tracks the state of background conversion jobs.
pub struct JobTracker {
    jobs: Mutex<HashMap<String, Job>>,
}

impl JobTracker {
    /// Instantiates an empty `JobTracker`.
    pub fn new() -> Self {
        JobTracker { jobs: Mutex::new(HashMap::new()) }
    }

    /// Registers a new running job, dropping stale finished jobs to keep the tracker bounded.
    pub fn insert(&self, job_id: &str, output_path: &str) {
        let mut jobs = self.jobs.lock().unwrap();
        let stale_threshold = SystemTime::now() - Duration::from_secs(600);
        jobs.retain(|_, job| job.state == "running" || job.created > stale_threshold);
        jobs.insert(job_id.to_string(), Job {
            state: "running".to_string(),
            detail: String::new(),
            output: output_path.to_string(),
            created: SystemTime::now(),
        });
    }

    /// Updates the state of a finished job.
    pub fn finish(&self, job_id: &str, state: &str, detail: String) {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(job) = jobs.get_mut(job_id) {
            job.state = state.to_string();
            job.detail = detail;
        }
    }

    /// Returns a clone of the job if it exists.
    pub fn get(&self, job_id: &str) -> Option<Job> {
        self.jobs.lock().unwrap().get(job_id).cloned()
    }

    /// Returns whether a running job is already writing to the given output file.
    pub fn is_running(&self, output_path: &Path) -> bool {
        let output = output_path.to_string_lossy().to_string();
        self.jobs.lock().unwrap().values().any(|job| job.state == "running" && job.output == output)
    }
}

/// Represents the state of a background conversion job.
#[derive(Clone, Serialize)]
pub struct Job {
    /// `running` while ffmpeg is executing, `done` on success, `failed` on error.
    pub state: String,
    /// Output path of the converted file on success, error message on failure.
    pub detail: String,
    #[serde(skip_serializing)]
    output: String,
    #[serde(skip_serializing)]
    created: SystemTime,
}

/// Validates a conversion request without running ffmpeg.
///
/// # Arguments
///
/// * `filepath` - Path to the media file that has to be converted.
/// * `target_format` - Format the file should be converted into.
///
/// # Returns
///
/// * `Ok(PathBuf)` - Output path if the request is valid.
/// * `Err(String)` - Reason for the rejection.
pub fn validate(filepath: &Path, target_format: &str) -> Result<PathBuf, String> {
    if !filepath.is_file() {
        return Err(format!("{:?} is not a valid file entry", filepath));
    }
    let current_format = match filepath.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => ext.to_lowercase(),
        None => return Err(format!("{:?} has no file extension", filepath)),
    };
    if !constant::VIDEO_FORMATS.contains(&current_format.as_str()) {
        return Err(format!("'{}' is not a convertible video format", current_format));
    }
    let target = validate_format(&current_format, target_format)?;
    let output_path = filepath.with_extension(&target);
    if output_path.exists() {
        return Err(format!(
            "Target file already exists: '{}'. Delete it first, the original file is never overwritten",
            output_path.to_string_lossy()
        ));
    }
    Ok(output_path)
}
