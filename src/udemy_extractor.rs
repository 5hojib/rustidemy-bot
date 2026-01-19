use anyhow::Result;
use scraper::{Html, Selector};

pub fn extract_main_description(description_html: &str, title: &str) -> String {
    let fragment = Html::parse_fragment(description_html);
    let selector = Selector::parse("p").unwrap();

    let mut combined_text = String::new();

    for element in fragment.select(&selector) {
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();

        if !text.is_empty() && !text.contains(title) && !text.starts_with("http") {
            combined_text = text;
            break;
        }
    }

    if combined_text.is_empty() {
        return "No description provided".to_string();
    }

    if let Some((before, after)) = combined_text.split_once("Published by:") {
        format!("{}\n\nPublished by: {}", before.trim(), after.trim())
    } else {
        combined_text
    }
}

use regex::Regex;

pub fn extract_thumbnail_url(description_html: &str) -> Option<String> {
    let re = Regex::new(r#"<img src="([^"]+)""#).unwrap();
    re.captures(description_html)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}

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
