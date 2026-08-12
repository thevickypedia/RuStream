# RuStream

[![made-with-rust][rust-logo]][rust-src-page]

[![crates.io][crates-logo]][crate]

[![build][gh-logo]][build]
[![docker][docker-logo]][ghcr]

#### Summary
[`RuStream`][repo] is a self-hosted streaming engine, that can render media files via authenticated sessions.

#### Docker Support
This fork ships with out-of-the-box Docker support via a multi-stage `Dockerfile` and `docker-compose.yaml`. The final image contains only the release binary, all configuration stays in `.env`. See the [Docker](#docker) section for a quick start.

### Installation

```shell
cargo add RuStream
```

### Usage
```rust,no_run
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
curl -o RuStream-Darwin-x86_64.tar.gz -LH "Accept: application/octet-stream" "https://github.com/Veucci/RuStream/releases/latest/download/RuStream-Darwin-x86_64.tar.gz"
```

###### Linux
```shell
curl -o RuStream-Linux-x86_64.tar.gz -LH "Accept: application/octet-stream" "https://github.com/Veucci/RuStream/releases/latest/download/RuStream-Linux-x86_64.tar.gz"
```

###### RaspberryPi
```shell
curl -o RuStream-RaspberryPi.tar.gz -LH "Accept: application/octet-stream" "https://github.com/Veucci/RuStream/releases/latest/download/RuStream-RaspberryPi.tar.gz"
```

###### Windows
```shell
curl -o RuStream-Windows-x86_64.zip -LH "Accept: application/octet-stream" "https://github.com/Veucci/RuStream/releases/latest/download/RuStream-Windows-x86_64.zip"
```
</details>

#### Environment Variables

**Optional**
- **authorization**: Dictionary of key-value pairs with `username` as key and `password` as value. Defaults to `{"user": "SuperSecurePass"}` when not provided, a warning is logged on startup.
- **media_source**: Source path for the files to be streamed. Defaults to `/data/media` when not provided, the directory is created if missing and a warning is logged on startup.
  > Files starting/ending with `_` _(underscore)_ and `.` _(dot)_ will be ignored
- **debug**: Boolean flag to enable debug level logging. Defaults to `false`
- **utc_logging**: Boolean flag to set timezone to UTC in the output logs. Defaults to `true`
- **media_host**: IP address to host the server. Defaults to `0.0.0.0`
- **media_port**: Port number to host the application. Defaults to `8000`
- **base_url**: URL path prefix under which the application is served, e.g. `/rustream`. Defaults to `/`
- **session_duration**: Time _(in seconds)_ each authenticated session should last. Defaults to `3600`
- **file_formats**: Vector of supported file formats. Defaults to `["*"]` (every file is listed)
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
- **ffmpeg_enabled**: Boolean flag to enable on-demand video conversion with ffmpeg. Defaults to `false`
  > ffmpeg is invoked **only** when a conversion is requested, it does not run in the background<br>
  > Converted files are written next to the originals, the originals are never modified or removed<br>
  > Requires `ffmpeg`/`ffprobe` binaries, bundled in the Docker image

> Checkout `.env.example` for a sample of all environment variables and `dotenv` usage.

### Docker

A multi-stage `Dockerfile` is included, the final image contains only the release binary and ffmpeg.

```shell
cp .env.example .env
mkdir media
docker compose up -d --build
```

The container listens on port `8000`, the port can be changed by setting `media_port` in `.env`. Media files are served from the `media/` directory, mounted to `/data/media` inside the container, so `media_source` in `.env` must be set to `/data/media`.

> The container runs as the host user (`UID`/`GID` from `.env`) so that files in the bind-mounted `media/` folder stay readable/writable. Set `UID` and `GID` to the user that owns the `media/` folder on the host (`id -u` and `id -g`).

Images are published to GHCR, tagged `stable` for `main` branch builds and `beta` for `dev` branch builds.

```shell
docker pull ghcr.io/veucci/rustream:stable
docker pull ghcr.io/veucci/rustream:beta
```

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

## License & copyright

&copy; Vignesh Rao

Licensed under the [MIT License][license]

[repo]: https://github.com/Veucci/RuStream
[license]: https://github.com/Veucci/RuStream/blob/main/LICENSE
[build]: https://github.com/Veucci/RuStream/actions/workflows/ci.yml
[rust-src-page]: https://www.rust-lang.org/
[rust-logo]: https://img.shields.io/badge/Made%20with-Rust-black?style=for-the-badge&logo=Rust
[gh-logo]: https://github.com/Veucci/RuStream/actions/workflows/ci.yml/badge.svg
[docker-logo]: https://img.shields.io/badge/Docker-Ready-2496ED?style=for-the-badge&logo=Docker&logoColor=white
[ghcr]: https://github.com/users/Veucci/packages/container/package/rustream
[crate]: https://crates.io/crates/RuStream
[crates-logo]: https://img.shields.io/crates/v/RuStream.svg
[docs]: https://docs.rs/RuStream/latest/rustream/
