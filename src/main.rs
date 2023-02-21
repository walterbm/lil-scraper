use atty::Stream;
use clap::Parser;
use error::ScrapeError;
use hyper::Client;
use log::info;
use regex::Regex;
use scraper::Scraper;
use std::io::BufRead;
use std::process;
use std::time::Instant;
use tokio::sync::mpsc;

use crate::printer::{Printer, TablePrinter, TextPrinter};

mod error;
mod printer;
mod scraper;

const CHANNEL_BUFFER: usize = 500;

/// CLI tool to quickly scrape short snippets of text data from multiple HTTP sources
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Regex pattern to use in search including a target group to extract
    ///
    /// for example:
    ///     --pattern '<title>(.*)</title>'
    ///     --pattern '<meta name="og:site_name" content="([^"]+)"'
    #[arg(short, long)]
    pattern: String,

    /// Timeout (in seconds)
    #[arg(short, long, default_value_t = 5)]
    timeout: u64,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();
    let regex =
        Regex::new(&args.pattern).expect("Error: pattern must be a valid regular expression!");

    let stdin = std::io::stdin();
    if atty::is(Stream::Stdin) {
        eprintln!("Error: stdin not redirected");
        process::exit(exitcode::IOERR);
    }

    type ChannelData = (String, Result<Option<String>, ScrapeError>);
    let (tx, mut rx) = mpsc::channel::<ChannelData>(CHANNEL_BUFFER);
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_or_http()
        .enable_http1()
        .enable_http2()
        .build();

    let client = Client::builder().build::<_, hyper::Body>(https);

    let scraper = Scraper::new(regex, args.timeout, client);

    stdin.lock().lines().for_each(|line| {
        let line = line.expect("Error: Could not read line from stdin");
        let tx = tx.clone();
        let scraper = scraper.clone();

        tokio::spawn(async move {
            let now = Instant::now();
            let result = {
                let uri = line.parse().map_err(|_| ScrapeError::InvalidURI)?;

                scraper.scrape(uri).await
            };
            info!("Completed scrape for {} in {:?}", line, now.elapsed());
            tx.send((line.clone(), result))
                .await
                .map_err(|_| ScrapeError::SendError)
        });
    });

    // close the channel
    drop(tx);

    let mut output: Box<dyn Printer> = if atty::is(Stream::Stdout) {
        Box::new(TablePrinter::new())
    } else {
        Box::new(TextPrinter::new())
    };

    while let Some(res) = rx.recv().await {
        match res {
            (url, Ok(Some(matches))) => output.success(&url, &matches),
            (url, Ok(None)) => output.error(&url, ScrapeError::NoMatch.message()),
            (url, Err(e)) => output.error(&url, e.message()),
        };
    }

    output.finish();
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}
