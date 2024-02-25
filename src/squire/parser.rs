use std::env;
use std::process::exit;
use crate::constant::Cargo;

/// Parses and returns the command-line arguments for RuStream.
///
/// # Returns
///
/// A String notion of the `env_file` argument.
pub fn arguments(cargo: &Cargo) -> String {
    let args: Vec<String> = env::args().collect();

    let mut version = false;
    let mut env_file = String::new();

    // Loop through the command-line arguments and parse them.
    let mut i = 1; // Start from the second argument (args[0] is the program name).
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                let helper = "RuStream takes the arguments, --env_file and --version/-v\n\n\
                --env_file: Custom filename to load the environment variables. Defaults to '.env'\n\
                --version: Get the package version.\n".to_string();
                println!("Usage: {} [OPTIONS]\n\n{}", args[0], helper);
                exit(0)
            }
            "-V" | "-v" | "--version" => {
                version = true;
            }
            "--env_file" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    env_file = args[i].clone();
                } else {
                    println!("--env_file requires a value.");
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
        println!("{} {}", &cargo.pkg_name, &cargo.pkg_version);
        exit(0)
    }
    env_file
}
