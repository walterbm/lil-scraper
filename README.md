# Lil' Scraper

`lil-scraper` is a small CLI to quickly scrape short snippets of text data from multiple HTTP sources.

<p align="center">
  <img width="700" src="https://raw.githubusercontent.com/walterbm/lil-scraper/main/demo/shell.svg">
</p>

## Installation

## Usage

Feed `lil-scraper` a list of urls (one url per line) from stdin and pass in a regular expression pattern to search for:

```
cat urls.txt | lil-scraper  --pattern '<i lang="es">([^<]+)</i>'
```

## License

Distributed under the MIT License. See `LICENSE` for more information.
