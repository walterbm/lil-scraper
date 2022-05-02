use std::time::Duration;

use super::ScrapeError;
use hyper::body;
use hyper::Client;
use hyper::StatusCode;
use hyper::Uri;
use regex::Regex;
use tokio::task;
use tokio::time;

#[derive(Clone)]
pub struct Scraper<C> {
    regex: Regex,
    timeout: u64,
    client: Client<C>,
}

impl<C: 'static> Scraper<C>
where
    C: hyper::client::connect::Connect + std::clone::Clone + std::marker::Send + std::marker::Sync,
{
    pub fn new(regex: Regex, timeout: u64, client: hyper::Client<C>) -> Self {
        Scraper {
            regex,
            client,
            timeout,
        }
    }

    pub async fn scrape(&self, uri: Uri) -> Result<Option<String>, ScrapeError> {
        let data = self.request(uri).await?;
        Ok(self.search(data).await)
    }

    async fn request(&self, uri: Uri) -> Result<String, ScrapeError> {
        let data =
            match time::timeout(Duration::from_secs(self.timeout), self.client.get(uri)).await {
                Ok(result) => result.map_err(|_| ScrapeError::RequestFailed),
                Err(_) => Err(ScrapeError::RequestTimeout),
            }?;

        if data.status() != StatusCode::OK {
            return Err(ScrapeError::RequestFailed);
        }

        let bytes = body::to_bytes(data)
            .await
            .map_err(|_| ScrapeError::InvalidResponse)?;

        String::from_utf8(bytes.to_vec()).map_err(|_| ScrapeError::InvalidResponse)
    }

    async fn search(&self, data: String) -> Option<String> {
        let re = self.regex.clone();
        task::spawn_blocking(move || {
            re.captures(&data)
                .map(|m| m.get(1).map_or("".to_string(), |m| m.as_str().to_string()))
        })
        .await
        .ok()?
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::ScrapeError, scraper::Scraper};
    use regex::Regex;

    mock_connector!(MockResponses {
        "https://test.com" => "HTTP/1.1 200 OK\r\n\
                               \r\n\
                               High up ledges are out of reach, a jump to get there I'll now teach! Choose your spot with the greatest care, only one jump for bird and bear!"
        "https://bad.com" => "HTTP/1.1 400 Bad Request\r\n\
                              \r\n\
                              "

    });

    #[tokio::test]
    async fn test_scrape_extracts_matching_data() {
        let client = hyper::Client::builder().build::<_, hyper::Body>(MockResponses::default());

        let scraper = Scraper {
            client,
            regex: Regex::new(r"only one jump for (bird and bear)!").unwrap(),
            timeout: 5,
        };

        assert_eq!(
            scraper.scrape("https://test.com".parse().unwrap()).await,
            Ok(Some("bird and bear".to_string()))
        );
    }

    #[tokio::test]
    async fn test_scrape_returns_none_when_not_found() {
        let client = hyper::Client::builder().build::<_, hyper::Body>(MockResponses::default());

        let scraper = Scraper {
            client,
            regex: Regex::new(r"(/d)").unwrap(),
            timeout: 5,
        };

        assert_eq!(
            scraper.scrape("https://test.com".parse().unwrap()).await,
            Ok(None)
        );
    }

    #[tokio::test]
    async fn test_scrape_returns_error_when_request_fails() {
        let client = hyper::Client::builder().build::<_, hyper::Body>(MockResponses::default());

        let scraper = Scraper {
            client,
            regex: Regex::new(r"(/d)").unwrap(),
            timeout: 5,
        };

        assert_eq!(
            scraper.scrape("https://bad.com".parse().unwrap()).await,
            Err(ScrapeError::RequestFailed)
        );
    }
}
