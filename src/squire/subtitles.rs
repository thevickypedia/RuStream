use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::result::Result;

/// Converts an SRT file to VTT format.
///
/// # Arguments
///
/// * `filename` - The path to the input SRT file.
///
/// # Returns
///
/// A boolean indicating whether the conversion was successful.
pub fn srt_to_vtt(filepath: &PathBuf) -> Result<bool, String> {
    if filepath.extension().and_then(OsStr::to_str) != Some("srt") {
        return Ok(false);
    }

    let output_file = filepath.with_extension("vtt");
    let mut rf = match File::open(filepath) {
        Ok(file) => file,
        Err(err) => return Err(format!("Error opening file: {}", err)),
    };

    let mut srt_content = String::new();
    if let Err(err) = rf.read_to_string(&mut srt_content) {
        return Err(format!("Error reading file: {}", err));
    }

    let srt_content = srt_content.replace(',', ".");
    let srt_content = srt_content.replace(" --> ", "-->");

    let mut vtt_content = String::from("WEBVTT\n\n");
    let subtitle_blocks: Vec<&str> = srt_content.trim().split("\n\n").collect();

    for block in subtitle_blocks {
        let lines: Vec<&str> = block.split('\n').collect();
        let timecode = lines[1];
        let text = lines[2..].join("\n");
        vtt_content.push_str(&format!("{}\n{}\n\n", timecode, text));
    }

    let mut wf = match File::create(output_file) {
        Ok(file) => file,
        Err(err) => return Err(format!("Error creating output file: {}", err)),
    };

    if let Err(err) = wf.write_all(vtt_content.as_bytes()) {
        return Err(format!("Error writing to output file: {}", err));
    }

    if let Err(err) = wf.flush() {
        return Err(format!("Error flushing output file: {}", err));
    }

    Ok(true)
}
