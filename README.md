# Lil' Scraper

`lil-scraper` is a small CLI to quickly scrape short snippets of text data from multiple HTTP sources.

<p align="center">
  <img width="700" src="https://raw.githubusercontent.com/walterbm/lil-scraper/main/demo/shell.svg">
</p>

## Installation

Quickest way to install the `lil-scraper` is using the [`install.sh`](https://github.com/walterbm/lil-scraper/blob/gh-pages/install.sh) script:

```bash
curl -LSfs https://walterbm.github.io/lil-scraper/install.sh | sh -s
```

Alternatively you can download the most recent binary from the [Github release artifacts](https://github.com/walterbm/lil-scraper/releases)

## Usage

Feed `lil-scraper` a list of urls (one url per line) from stdin and pass in a regular expression pattern to search for:

```bash
cat urls.txt | lil-scraper  --pattern '<i lang="es">([^<]+)</i>'
```

This is roughly equivalent to running using `xargs` however lil-scraper will run significantly faster thanks to the [tokio async runtime](https://tokio.rs/).

```bash
cat urls.txt | xargs -P 0 curl | grep -ioE '<i lang="es">([^<]+)</i>'
```

## License

Distributed under the MIT License. See `LICENSE` for more information.
