# Lil' Scraper

`lil-scraper` is a small CLI to quickly scrape short snippets of text data from multiple HTTP sources.

<p align="center">
  <img width="700" src="https://raw.githubusercontent.com/walterbm/lil-scraper/main/demo/shell.svg">
</p>

## Installation

Installation is currently a very manual process and will require explicitly selecting the desired version and target architecture. For example to install for Intel Macs run the following commands:

```bash
VERSION=v0.1.4 \
  && ARCH=x86_64-apple-darwin \
  && mkdir -p /tmp/lil-scraper \
  && curl -sL https://github.com/walterbm/lil-scraper/releases/download/$VERSION/lil-scraper-$VERSION-$ARCH.tar.gz \
  | tar -C /tmp/lil-scraper -xz \
  && install -m 755 /tmp/lil-scraper/lil-scraper-$VERSION-$ARCH/lil-scraper /usr/local/bin \
  && rm -rf /tmp/lil-scraper
```

## Usage

Feed `lil-scraper` a list of urls (one url per line) from stdin and pass in a regular expression pattern to search for:

```bash
cat urls.txt | lil-scraper  --pattern '<i lang="es">([^<]+)</i>'
```

This is roughly equivalent to running using `xargs` however lil-scraper will run significantly faster thanks to the tokio async runtime.

```bash
cat urls.txt | xargs -P 0 curl | grep -ioE '<i lang="es">([^<]+)</i>'
```

## License

Distributed under the MIT License. See `LICENSE` for more information.
