/// Module for the commandline interface creation kit that parses the commandline arguments
pub mod parser;
/// Module for the web data configuration that holds the secrets required by the application.
pub mod settings;
/// Module that initializes the logger and loads the configuration into a dedicated Struct.
pub mod startup;
/// Module for the functions that handle encryption/encoding and decryption/decoding.
pub mod secure;
/// Module for the function that logs the incoming connection information.
pub mod logger;
/// Module for the functions that yield an ASCII art to print during startup.
pub mod ascii_art;
/// Module for the CORS middleware configuration.
pub mod middleware;
/// Module for the function that converts the subtitles from `srt` to `vtt` file format.
pub mod subtitles;
/// Module for the functions that scan the video source and render the filenames as struct.
pub mod content;
/// Module that handles the authentication and
pub mod authenticator;
