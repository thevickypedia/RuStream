use std::env;
use std::process::exit;

/// Represents the command-line arguments for RuStream.
pub struct Args {
    /// JSON filename where the config information is stored.
    pub debug: bool,
    pub filename: String,
}

/// Parses and returns the command-line arguments for RuStream.
///
/// # Returns
///
/// An `Args` struct containing parsed command-line arguments.
pub fn arguments() -> Args {
    let args: Vec<String> = env::args().collect();

    let mut debug = false;

    let mut version = false;
    let mut filename = String::new();

    // Loop through the command-line arguments and parse them.
    let mut i = 1; // Start from the second argument (args[0] is the program name).
    while i < args.len() {
        match args[i].as_str() {
            "debug" => {
                debug = true;
            }
            "-h" | "--help" => {
                let helper = "RuStream takes the arguments, debug, \
                --filename/-f and --version/-v\n\n\
        debug: Optional flag to enable debug level logging\n\
        --filename: JSON filename with the following arguments as a feed.\n\
        --version: Get the package version.\n\
        \nJSON file content\nMandatory Args:\n
        authorization: Dictionary of key-value pairs with username as key and password as value.
        video_source: Source path for video files.\n\n\
        Optional Args:\n
        video_host: IP address to host the video. Defaults to 127.0.0.1
        video_port: Port number to host the application. Defaults to 8000
        file_formats: Sequence of supported video file formats. Defaults to (.mp4, .mov)
        workers: Number of workers to spin up the server. Defaults to the number of physical cores.
        website: List of websites (supports regex) to add to CORS configuration. Required only if tunneled via CDN\n"
                    .to_string();
                println!("Usage: {} [OPTIONS]\n\n{}", args[0], helper);
                exit(0)
            }
            "-V" | "-v" | "--version" => {
                version = true;
            }
            "--filename" | "-f" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    filename = args[i].clone();
                } else {
                    println!("--filename requires a value.");
                    exit(1)
                }
            }
            _ => {
                println!("Unknown argument: {}", args[i]);
                exit(1)
            }
        }
        i += 1;
    }
    if version {
        const PKG_NAME: &str = env!("CARGO_PKG_NAME");
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("{} {}", PKG_NAME, VERSION);
        exit(0)
    }

    Args {
        debug,
        filename,
    }
}
