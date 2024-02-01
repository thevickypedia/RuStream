use std::env;
use std::process::exit;

pub struct Args {
    pub debug: bool,
    pub filename: String,
}


pub fn arguments() -> Args {
    let args: Vec<String> = env::args().collect();

    let usage = format!(
        "flags:\n\
        \tdebug:{}Enable debug mode for detailed logging\n\
        \tversion:{}Get version of the package\n\n\
        arguments:\n\
        \t-f / --filename{}JSON filename where the config information is stored \
        (use -h/--help to vide the args required in the JSON file)\n",
        " ".repeat(6), " ".repeat(8), " ".repeat(8)
    );
    if args.is_empty() {
        // If no arguments are provided, display usage instructions
        println!("Usage: {} [OPTIONS]\n\n{}", args[0], usage);
        exit(1)
    }

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
                let helper = "\nMandatory Args:\n
        authorization: Dictionary of key-value pairs with username as key and password as value.\n
        video_source: Source path for video files.\n\n\
        Optional Args:\n
        video_host: IP address to host the video. Defaults to 127.0.0.1\n
        video_port: Port number to host the application. Defaults to 8000\n
        file_formats: Sequence of supported video file formats. Defaults to (.mp4, .mov)\n
        workers: Number of workers to spin up the uvicorn server. Defaults to 1\n
        website: List of websites (supports regex) to add to CORS configuration. Required only if tunneled via CDN\n
        auto_thumbnail: Boolean flag to auto generate thumbnail images for preview. Defaults to true\n\n".to_string();
                println!("{}", helper);
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
