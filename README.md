# RuStream

[![made-with-rust][rust-logo]][rust-src-page]

[![crates.io][crates-logo]][crate]

[![build][gh-logo]][build]
[![none-shall-pass][nsp-logo]][nsp]

#### Summary
[`RuStream`][repo] is a self-hosted streaming engine, that can render videos via authenticated sessions.

### Installation

```shell
cargo add RuStream
```

### Usage
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

#### Arguments
- `debug` - Enable debug level logging

#### Flags
- `--filename` / `-f` - Filename (JSON) for the secrets' config. Defaults to `config.json`
- `--version` / `-v` - Get package version

#### Config file
[RuStream][repo] requires a JSON file with secrets loaded as key-value paris.

**Mandatory**
- **authorization**: Dictionary of key-value pairs with `username` as key and `password` as value.
- **video_source**: Source path for video files.
> Files starting with `_` _(underscore)_ and `.` _(dot)_ will be ignored

**Optional**
- **video_host**: IP address to host the video. Defaults to `127.0.0.1` / `localhost`
- **video_port**: Port number to host the application. Defaults to `8000`
- **session_duration**: Time _(in seconds)_ each authenticated session should last. Defaults to `3600`
- **file_formats**: Vector of supported video file formats. Defaults to `[.mp4, .mov]`
- **workers**: Number of workers to spin up for the server. Defaults to the number of physical cores.
- **max_connections**: Maximum number of concurrent connections per worker. Defaults to `3`
- **websites**: Vector of websites (_supports regex_) to add to CORS configuration. _Required only if tunneled via CDN_
- **key_file**: Path to the private key file for SSL certificate. Defaults to `None`
- **cert_file**: Path to the full chain file for SSL certificate. Defaults to `None`
- **secure_session**: Boolean flag to secure the cookie `session_token`. Defaults to `false`
> If `SECURE_SESSION` to set to `true`, the cookie `session_token` will only be sent via HTTPS<br>
> This means that the server can **ONLY** be hosted via `HTTPS` or `localhost`

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

### Cargo Docs - Official Runbook
[https://docs.rs/RuStream/latest/rustream/][docs]

**Generator**
```shell
cargo doc --document-private-items --no-deps
```

### GitHub Wiki - Project Insights
[https://github.com/thevickypedia/RuStream/wiki][gh-wiki]

## Linting
### Requirement
```shell
rustup component add clippy
```
### Usage
```shell
cargo clippy --no-deps --fix
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
[nsp-logo]: https://github.com/thevickypedia/RuStream/actions/workflows/none.yml/badge.svg
[nsp]: https://github.com/thevickypedia/RuStream/actions/workflows/none.yml
[crate]: https://crates.io/crates/RuStream
[gh-checks]: https://github.com/thevickypedia/RuStream/actions/workflows/rust.yml
[crates-logo]: https://img.shields.io/crates/v/RuStream.svg
[gh-wiki]: https://github.com/thevickypedia/RuStream/wiki
[docs]: https://docs.rs/RuStream/latest/rustream/
