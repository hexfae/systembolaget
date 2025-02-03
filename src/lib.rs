#![allow(clippy::missing_errors_doc)]

use bon::bon;
use error::{Error, Result};
use regex::Regex;
use reqwest::Client;
use std::collections::HashMap;
use strum_macros::Display;
use systembolaget::{Assortment, Product};

pub mod error;
pub mod systembolaget;

pub const URL_BASE: &str = "https://systembolaget.se";
pub const API_BASE: &str = "https://api-extern.systembolaget.se";

pub struct Systembolaget {
    pub client: Client,
    pub api_key: Option<String>,
    pub products: HashMap<String, Product>,
}

#[derive(Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SortDirection {
    #[strum(to_string = "Stigande (lägst först)")]
    Ascending,
    #[strum(to_string = "Fallande (högst först)")]
    Descending,
}

#[derive(Display, Clone, Copy)]
pub enum SortBy {
    // i don't know what this actually is
    // #[strum(to_string = "Poäng")]
    // Score,
    #[strum(to_string = "Pris")]
    Price,
    #[strum(to_string = "Namn")]
    Name,
    #[strum(to_string = "Volym")]
    Volume,
    #[strum(to_string = "Årgång")]
    Vintage,
    #[strum(to_string = "Säljstart")]
    ProductLaunchDate,
    #[strum(to_string = "ml/kr")]
    MlPerCrown,
    #[strum(to_string = "kr/ml")]
    CrownsPerMl,
}

#[bon]
impl Systembolaget {
    #[must_use]
    #[builder]
    pub fn new(
        client: Option<Client>,
        api_key: Option<String>,
        products: Option<HashMap<String, Product>>,
    ) -> Self {
        Self {
            client: client.unwrap_or_default(),
            api_key,
            products: products.unwrap_or_default(),
        }
    }

    #[must_use]
    pub fn with_api_key(api_key: impl Into<String>) -> Self {
        Self::builder().api_key(api_key.into()).build()
    }

    #[must_use]
    pub fn from_assortment(assortment: Assortment) -> Self {
        Self::builder().products(assortment.into()).build()
    }

    pub async fn fetch_api_key(&mut self) -> Result<()> {
        let app_bundle_path_regex = Regex::new(r#"<script src="([^"]+_app-[^"]+.js)""#)?;
        let home = self.client.get(URL_BASE).send().await?.text().await?;
        let Some(caps) = app_bundle_path_regex.find(&home) else {
            return Err(Error::FetchApiKey);
        };
        let Some(stripped_prefix) = caps.as_str().strip_prefix("<script src=\"") else {
            return Err(Error::FetchApiKey);
        };
        let Some(stripped_suffix) = stripped_prefix.strip_suffix('"') else {
            return Err(Error::FetchApiKey);
        };
        let script = self
            .client
            .get(format!("{URL_BASE}{stripped_suffix}"))
            .send()
            .await?
            .text()
            .await?;
        let api_token_regex = Regex::new(r#"NEXT_PUBLIC_API_KEY_APIM:"([^"]+)""#)?;
        let Some(api_key) = api_token_regex.find(&script) else {
            return Err(Error::FetchApiKey);
        };
        let Some(stripped_prefix) = api_key.as_str().strip_prefix("NEXT_PUBLIC_API_KEY_APIM:\"")
        else {
            return Err(Error::FetchApiKey);
        };
        let Some(stripped_suffix) = stripped_prefix.strip_suffix('"') else {
            return Err(Error::FetchApiKey);
        };
        self.api_key = Some(stripped_suffix.to_string());
        Ok(())
    }

    pub async fn fetch_page(
        &self,
        page: u16,
        sort_by: SortBy,
        sort_direction: SortDirection,
    ) -> Result<Assortment> {
        let Some(ref api_key) = self.api_key else {
            return Err(Error::NoApiKey);
        };
        let url = format!("{API_BASE}/sb-api-ecommerce/v1/productsearch/search?page={page}&size=30&sortBy={sort_by}&sortDirection={sort_direction}");
        let request = self
            .client
            .get(url)
            .header("ocp-apim-subscription-key", api_key);
        let response = request.send().await?;
        let text = response.text().await?;
        let assortment = serde_json::from_str::<Assortment>(&text)?;
        Ok(assortment)
    }

    pub async fn fetch_entire_assortment(
        &mut self,
        sort_by: SortBy,
        sort_direction: SortDirection,
    ) -> Result<()> {
        let mut page = 1;
        let assortment = self.fetch_page(page, sort_by, sort_direction).await?;
        let total_results = assortment.metadata.doc_count as usize;
        self.products.extend(assortment);
        while (self.products.len()) < total_results {
            page += 1;
            let assortment = self.fetch_page(page, sort_by, sort_direction).await?;
            self.products.extend(assortment);
        }
        Ok(())
    }
}
