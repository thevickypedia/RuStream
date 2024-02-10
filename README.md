# RuStream

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-black?style=for-the-badge&logo=Rust)][rust]

[![build](https://github.com/thevickypedia/RuStream/actions/workflows/release.yml/badge.svg)][build]

#### Summary
[`RuStream`][1] is an application written in Rust to stream videos using Actix API via authenticated sessions.

## Usage

#### Download Executable
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

#### Arguments
- `debug` - Enable debug level logging

#### Flags
- `--filename` / `-f` - Filename (JSON) for the secrets config

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
[build]: https://github.com/thevickypedia/RuStream/actions/workflows/release.yml
[rust]: https://www.rust-lang.org/
