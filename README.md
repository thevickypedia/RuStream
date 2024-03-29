# RuStream

[![made-with-rust][rust-logo]][rust-src-page]

[![crates.io][crates-logo]][crate]

[![build][gh-logo]][build]
[![none-shall-pass][nsp-logo]][nsp]

#### Summary
[`RuStream`][repo] is a self-hosted streaming engine, that can render media files via authenticated sessions.

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
            println!("RuStream session has ended")
        }
        Err(err) => {
            eprintln!("Error starting RuStream: {}", err)
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

#### Environment Variables

**Mandatory**
- **authorization**: Dictionary of key-value pairs with `username` as key and `password` as value.
- **media_source**: Source path for the files to be streamed.
  > Files starting/ending with `_` _(underscore)_ and `.` _(dot)_ will be ignored

**Optional**
- **debug**: Boolean flag to enable debug level logging. Defaults to `false`
- **utc_logging**: Boolean flag to set timezone to UTC in the output logs. Defaults to `true`
- **media_host**: IP address to host the server. Defaults to `127.0.0.1` / `localhost`
- **media_port**: Port number to host the application. Defaults to `8000`
- **session_duration**: Time _(in seconds)_ each authenticated session should last. Defaults to `3600`
- **file_formats**: Vector of supported file formats. Defaults to `[mp4, mov, jpg, jpeg]`
- **workers**: Number of workers to spin up for the server. Defaults to the number of physical cores.
- **max_connections**: Maximum number of concurrent connections per worker. Defaults to `3`
- **max_payload_size**: Maximum size of files that can be uploaded from the UI. Defaults to `100 MB`
  > Input should be in the format, `10 MB`, `3 GB` - _inputs are case insensitive_
- **websites**: Vector of websites (_supports regex_) to add to CORS configuration. _Required only if tunneled via CDN_
- **key_file**: Path to the private key file for SSL certificate. Defaults to `None`
- **cert_file**: Path to the full chain file for SSL certificate. Defaults to `None`
- **secure_session**: Boolean flag to secure the cookie `session_token`. Defaults to `false`
  > If `secure_session` is to set to `true`, the cookie `session_token` will only be sent via HTTPS<br>
  > This means that the server can **ONLY** be hosted via `HTTPS` or `localhost`

> Checkout [GitHub Wiki][gh-wiki-env] for more information about environment variables and `dotenv` usage.

## Crate
[https://crates.io/crates/RuStream][crate]

### Cargo Docs - Official Runbook
[https://docs.rs/RuStream/latest/rustream/][docs]

**Generator**
```shell
cargo doc --document-private-items --no-deps
```

## Linting
### Requirement
```shell
rustup component add clippy
```
### Usage
```shell
cargo clippy --no-deps --fix
```

## GitHub Wiki - Project Insights
[https://github.com/thevickypedia/RuStream/wiki][gh-wiki]

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
[gh-wiki-env]: https://github.com/thevickypedia/RuStream/wiki/Environment-Variables
[docs]: https://docs.rs/RuStream/latest/rustream/
