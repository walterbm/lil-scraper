use super::ScrapeError;
use hyper::{body, Client, Uri};
use regex::Regex;
use std::time::Duration;
use tokio::time;

#[derive(Clone)]
pub struct Scraper<C> {
    regex: Regex,
    timeout: u64,
    client: Client<C>,
}

impl<C> Scraper<C>
where
    C: hyper::client::connect::Connect + Clone + Send + Sync + 'static,
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
        Ok(self.search(&data).map(|s| s.to_string()))
    }

    async fn request(&self, uri: Uri) -> Result<String, ScrapeError> {
        let data =
            match time::timeout(Duration::from_secs(self.timeout), self.client.get(uri)).await {
                Ok(result) => result.map_err(|_| ScrapeError::RequestFailed),
                Err(_) => Err(ScrapeError::RequestTimeout),
            }?;

        if !data.status().is_success() {
            return Err(ScrapeError::RequestFailed);
        }

        let bytes = body::to_bytes(data)
            .await
            .map_err(|_| ScrapeError::InvalidResponse)?;

        Ok(String::from_utf8_lossy(&bytes).into_owned())
    }

    fn search<'a>(&self, data: &'a str) -> Option<&'a str> {
        self.regex
            .captures(data)
            .and_then(|m| m.get(1))
            .map(|m| m.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::ScrapeError, scraper::Scraper};
    use regex::Regex;
    use yup_hyper_mock::*;

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
