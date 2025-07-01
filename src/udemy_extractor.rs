use anyhow::Result;
use scraper::{Html, Selector};

pub async fn extract_udemy_url(url: &str) -> Result<reqwest::Url> {
    let last_part = url.split('/').next_back().unwrap_or("");
    let converted_url = format!("https://www.discudemy.com/go/{last_part}#google_vignette");

    let client = reqwest::Client::new();
    let response = client.get(&converted_url).send().await?;
    let body = response.text().await?;

    let document = Html::parse_document(&body);
    let link_selector = Selector::parse("a[href]").unwrap();

    let udemy_links: Vec<String> = document
        .select(&link_selector)
        .filter_map(|element| {
            let href = element.value().attr("href")?;
            if href.contains("udemy.com") && !href.contains("discudemy.com") {
                Some(href.to_string())
            } else {
                None
            }
        })
        .collect();

    let udemy_url_str = udemy_links
        .first()
        .cloned()
        .unwrap_or_else(|| url.to_string());

    Ok(udemy_url_str.parse::<reqwest::Url>()?)
}
