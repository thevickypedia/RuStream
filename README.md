# RuStream

[![made-with-rust][rust-logo]][rust-src-page]

[![build][gh-logo]][build]

[![crates.io][crates-logo]][crate]

**Platform Supported**

[![platform][platform]][cross-platform]

#### Summary
[`RuStream`][repo] is a self-hosted streaming engine, that can render videos via authenticated sessions.

## Usage

<details>
<summary><strong>Download OS specific Executable</strong></summary>

###### macOS
```shell
curl -o RuStream-Darwin-x86_64.tar.gz -LH "Accept: application/octet-stream" "https://github.com/thevickypedia/RuStream/releases/latest/download/RuStream-Darwin-x86_64.tar.gz"
```

###### Linux
```shell
curl -o RuStream-Linux-x86_64.tar.gz -LH "Accept: application/octet-stream" "https://github.com/thevickypedia/RuStream/releases/latest/download/RuStream-Linux-x86_64.tar.gz"
```

###### RaspberryPi
```shell
curl -o RuStream-RaspberryPi.tar.gz -LH "Accept: application/octet-stream" "https://github.com/thevickypedia/RuStream/releases/latest/download/RuStream-RaspberryPi.tar.gz"
```

###### Windows
```shell
curl -o RuStream-Windows-x86_64.zip -LH "Accept: application/octet-stream" "https://github.com/thevickypedia/RuStream/releases/latest/download/RuStream-Windows-x86_64.zip"
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
            println!("RuStream session terminated")
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
[RuStream][repo] requires a JSON file with secrets loaded as key-value paris.

<details>
<summary><i><strong>Sample content of JSON file</strong></i></summary>

```json
{
  "authorization": {"rustic":  "S0m3rAn0mP@ssW0rD"},
  "video_source": "/Users/hannibal/Downloads/stream",
  "video_port": 5883,
  "file_formats": ["mov", "mp4", "mkv"],
  "workers": 10
}
```
</details>

## Crate
[https://crates.io/crates/RuStream][crate]

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

Licensed under the [MIT License][license]

[repo]: https://github.com/thevickypedia/RuStream
[license]: https://github.com/thevickypedia/RuStream/blob/main/LICENSE
[build]: https://github.com/thevickypedia/RuStream/actions/workflows/rust.yml
[rust-src-page]: https://www.rust-lang.org/
[rust-logo]: https://img.shields.io/badge/Made%20with-Rust-black?style=for-the-badge&logo=Rust
[gh-logo]: https://github.com/thevickypedia/RuStream/actions/workflows/rust.yml/badge.svg
[crate]: https://crates.io/crates/RuStream
[gh-checks]: https://github.com/thevickypedia/RuStream/actions/workflows/rust.yml
[crates-logo]: https://img.shields.io/crates/v/RuStream.svg
[platform]: https://img.shields.io/badge/Platform-Linux_|_macOS_|_Windows-blue?style=for-the-badge&logo=ubuntu
[cross-platform]: https://www.computerlanguage.com/results.php?definition=cross+platform
