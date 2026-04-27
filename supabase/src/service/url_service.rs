use regex::Regex;
use reqwest::Client;
use reqwest::Url;

use crate::error::custom_errors::UrlFetchError;
use crate::model::bookmark::PageMetadata;

pub async fn extract_page_metadata(
    client: &Client,
    raw_url: &str,
) -> Result<PageMetadata, UrlFetchError> {
    let url = Url::parse(raw_url)
        .map_err(|e| UrlFetchError::InvalidUrl(e.to_string()))?;

    let response = client
        .get(url.clone())
        .send()
        .await
        .map_err(|e| UrlFetchError::FetchError(e.to_string()))?;

    let body = response
        .text()
        .await
        .map_err(|e| UrlFetchError::FetchError(e.to_string()))?;

    let title_regex = Regex::new(r"(?is)<title>(.*?)</title>")
        .map_err(|e| UrlFetchError::FetchError(e.to_string()))?;

    let title = title_regex
        .captures(&body)
        .and_then(|captures| captures.get(1))
        .map(|value| value.as_str().trim().to_string())
        .filter(|value| !value.is_empty());

    let host = url
        .host_str()
        .map(|value| value.to_string())
        .ok_or_else(|| UrlFetchError::InvalidUrl("Url did not contain a host".to_string()))?;

    Ok(PageMetadata { host, title })
}