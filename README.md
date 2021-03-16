# DDns

Simple command line for [Duck DNS](https://www.duckdns.org/)

## Build

Build with: `cargo build`. Binary can be found in `target/debug/`.

## Usage

- Help: `ddns --help`
- Without ip: `ddns .ddns -d yourdomain -t 5893da4a-e049-4e65-8285-e56bf9cec729`
- With ip:    `ddns .ddns -d yourdomain -t 5893da4a-e049-4e65-8285-e56bf9cec729 -i 127.0.0.0`
