# Lil' Scraper

`lil-scraper` is a small CLI tool to quickly scrape short snippets of text data from multiple HTTP sources.

<p align="center">
  <img width="700" src="https://raw.githubusercontent.com/walterbm/lil-scraper/main/demo/shell.svg">
</p>

## Installation

Quickest way to install `lil-scraper` is using the provided [`install.sh`](https://github.com/walterbm/lil-scraper/blob/gh-pages/install.sh) script:

```bash
curl -LSfs https://walterbm.github.io/lil-scraper/install.sh | sh -s
```

Alternatively you can download the most recent binary from the [Github release artifacts](https://github.com/walterbm/lil-scraper/releases)

## Usage

Feed `lil-scraper` a list of urls (one url per line) from stdin and pass in a regular expression pattern with a capture group to search for:

```bash
cat urls.txt | lil-scraper  --pattern '<i lang="es">([^<]+)</i>'
```

This is roughly equivalent to running a similar command using `xargs` however `lil-scraper` will run **significantly** faster thanks to the [tokio async runtime](https://tokio.rs/).

```bash
cat urls.txt | xargs -P 0 curl | grep -ioE '<i lang="es">([^<]+)</i>'
```

## Command-line options

### `-p, --pattern` (required)

A [Rust regular expression](https://rustexp.lpil.uk/) with a capture group which will be used to search and extract text from the HTTP responses.

### `-t, --timeout` (defaults to 5 seconds)

The amount of time to wait for an HTTP response before disconnecting.

## Test

Run the test suite with:

```sh
cargo test
```

## License

Distributed under the MIT License. See `LICENSE` for more information.
