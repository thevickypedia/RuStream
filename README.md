# RuStream

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-black?style=for-the-badge&logo=Rust)][rust]

[![build](https://github.com/thevickypedia/RuStream/actions/workflows/rust.yml/badge.svg)][build]

#### Summary
[`RuStream`][1] is an application written in Rust to stream videos using Actix API via authenticated sessions.

## Usage

<details>
<summary><strong>Download OS specific Executable</strong></summary>

###### macOS
```shell
curl -o asset -LH "Accept: application/octet-stream" "https://github.com/thevickypedia/RuStream/releases/latest/download/RuStream-Darwin-x86_64.tar.gz"
```

###### Linux
```shell
curl -o asset -LH "Accept: application/octet-stream" "https://github.com/thevickypedia/RuStream/releases/latest/download/RuStream-Linux-x86_64.tar.gz"
```

###### RaspberryPi
```shell
curl -o asset -LH "Accept: application/octet-stream" "https://github.com/thevickypedia/RuStream/releases/latest/download/RuStream-RaspberryPi.tar.gz"
```

###### Windows
```shell
curl -o asset -LH "Accept: application/octet-stream" "https://github.com/thevickypedia/RuStream/releases/latest/download/RuStream-Windows-x86_64.zip"
```
</details>

<details>
<summary><strong>Add to existing project</strong></summary>

###### Sample main.rs
```rust
use rustream;

#[actix_rt::main]
async fn main() {
    match rustream::start().await {
        Ok(_) => {
            println!("Successfully served session")
        }
        Err(err) => {
            eprintln!("Error starting rustream: {}", err)
        }
    }
}
```

</details>

#### Arguments
- `debug` - Enable debug level logging

#### Flags
- `--filename` / `-f` - Filename (JSON) for the secrets config
- `--version` / `-v` - Get package version

#### Config file
[RuStream][1] requires a JSON file with secrets loaded as key-value paris.

<details>
<summary><i><strong>Sample content of JSON file</strong></i></summary>

```json
{
  "authorization": {"rustic":  "S0m3rAn0mP@ssW0rD"},
  "video_source": "/Users/hannibal/Downloads/stream",
  "video_port": 5883,
  "file_formats": [".mov", ".mp4", ".mkv"],
  "workers": 10
}
```
</details>

## Crate
https://crates.io/crates/RuStream

## Linting
### Requirement
```shell
rustup component add clippy
```
### Usage
```shell
cargo clippy --no-deps --fix --allow-dirty
```

## License & copyright

&copy; Vignesh Rao

Licensed under the [MIT License][2]

[1]: https://github.com/thevickypedia/RuStream
[2]: https://github.com/thevickypedia/RuStream/blob/main/LICENSE
[build]: https://github.com/thevickypedia/RuStream/actions/workflows/rust.yml
[rust]: https://www.rust-lang.org/
