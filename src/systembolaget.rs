// using multiple strum derives gives
// a warning for this, for some reason
#![allow(unreachable_patterns)]

use askama_axum::Template;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::BuildHasher};
use strum_macros::{Display, EnumString};
use stylic::link;

#[derive(Debug, Serialize, Deserialize, Clone, Template)]
#[template(path = "index.html")]
pub struct Assortment {
    pub metadata: Metadata,
    pub products: Vec<Product>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub doc_count: u16,
    pub full_assortment_doc_count: u16,
    // on the last page, next_page is -1
    pub next_page: i16,
    // on the first page, previous_page is -1
    pub previous_page: i16,
    pub total_pages: u16,
    pub price_range: Range,
    pub volume_range: Range,
    pub alcohol_percentage_range: Range,
    pub sugar_content_range: Range,
    pub sugar_content_gram_per_100ml_range: Range,
    pub did_you_mean_query: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Range {
    pub min: f32,
    pub max: f32,
}

#[allow(clippy::struct_field_names, clippy::struct_excessive_bools)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub alcohol_percentage: f32,
    pub assortment: String,
    pub assortment_text: String,
    pub bottle_text: String,
    pub category: Option<String>,
    pub category_level1: String,
    pub category_level2: String,
    pub category_level3: Option<String>,
    pub category_level4: Option<String>,
    pub color: Option<String>,
    pub country: String,
    pub custom_category_title: String,
    // always null?
    pub dish_points: Option<String>,
    pub ethical_label: Option<String>,
    pub grapes: Vec<String>,
    pub image_modules: ImageModule,
    pub images: Vec<Image>,
    pub is_bs_assortment: bool,
    pub is_climate_smart_packaging: bool,
    pub is_completely_out_of_stock: bool,
    pub is_discontinued: bool,
    pub is_ethical: bool,
    pub is_fs_assortment: bool,
    pub is_fs_ts_assortment: bool,
    pub is_kosher: bool,
    pub is_manufacturing_country: bool,
    pub is_news: bool,
    pub is_organic: bool,
    pub is_pa_assortment: bool,
    pub is_recommended_by_taste_profile: bool,
    pub is_regional_restricted: bool,
    pub is_supplier_temporary_not_available: bool,
    pub is_sustainable_choice: bool,
    pub is_temporary_out_of_stock: bool,
    pub is_ts_assortment: bool,
    pub is_ts_ls_assortment: bool,
    pub is_tse_assortment: bool,
    pub is_tss_assortment: bool,
    pub is_tst_assortment: bool,
    pub is_tsv_assortment: bool,
    pub is_web_launch: bool,
    pub origin_level1: Option<String>,
    pub origin_level2: Option<String>,
    // always null?
    pub other_selections: Option<String>,
    pub packaging_level1: Option<String>,
    pub price: f32,
    pub producer_name: Option<String>,
    pub product_id: String,
    pub product_launch_date: String,
    pub product_name_bold: String,
    pub product_name_thin: Option<String>,
    pub product_number: String,
    pub product_number_short: String,
    pub recycle_fee: f32,
    pub restricted_parcel_quantity: u8,
    pub seal: Option<String>,
    pub sell_start_time: String,
    pub sugar_content: u16,
    pub sugar_content_gram_per100ml: f32,
    pub supplier_name: Option<String>,
    pub taste: Option<String>,
    pub taste_clock_bitter: u8,
    pub taste_clock_body: u8,
    pub taste_clock_casque: u8,
    pub taste_clock_fruitacid: u8,
    pub taste_clock_group_bitter: Option<u8>,
    pub taste_clock_group_smokiness: Option<u8>,
    pub taste_clock_roughness: u8,
    pub taste_clock_smokiness: u8,
    pub taste_clock_sweetness: u8,
    pub taste_clocks: Vec<TasteClock>,
    pub taste_symbols: Vec<String>,
    pub usage: Option<String>,
    pub vintage: Option<String>,
    pub volume: f32,
    pub volume_text: String,
}

#[allow(clippy::struct_field_names)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub image_url: String,
    // always null?
    pub file_type: Option<String>,
    // always null?
    pub size: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageModule {
    pub product_id: Option<String>,
    pub thumbnail: Option<String>,
    pub sizes: Option<String>,
    pub extensions: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TasteClock {
    pub key: String,
    pub value: u8,
}

#[derive(Debug, Clone, Copy, Display, EnumString)]
pub enum ProductAssortment {
    #[strum(serialize = "Fast sortiment")]
    Fixed,
    #[strum(serialize = "Ordervaror")]
    OrderWare,
    #[strum(serialize = "Säsong")]
    Season,
    #[strum(serialize = "Tillfälligt sortiment")]
    Temporary,
    #[strum(serialize = "Lokalt & Småskaligt")]
    Local,
    #[strum(serialize = "Webblanseringar")]
    Web,
}

#[derive(Debug, Display, EnumString)]
pub enum ProductType {
    #[strum(serialize = "Vin")]
    #[strum(to_string = "Vin")]
    Wine,
    #[strum(serialize = "Öl")]
    #[strum(to_string = "Öl")]
    Beer,
    #[strum(serialize = "Sprit")]
    #[strum(to_string = "Sprit")]
    Spirit,
    #[strum(serialize = "Cider & blanddrycker")]
    #[strum(to_string = "Cider & blanddrycker")]
    Cider,
    #[strum(serialize = "Alkoholfritt")]
    #[strum(to_string = "Alkoholfritt")]
    AlcoholFree,
}

impl Product {
    #[must_use]
    pub fn to_ml_per_crown(&self) -> f32 {
        (self.volume * (self.alcohol_percentage / 100.0)) / self.price
    }

    #[must_use]
    pub fn to_crowns_per_ml(&self) -> f32 {
        self.price / (self.volume * (self.alcohol_percentage / 100.0))
    }

    #[must_use]
    pub fn link_display_price(&self) -> String {
        format!(
            "{}",
            link(
                format!("{}: {}:-", self.product_name_bold, self.price),
                format!(
                    "https://systembolaget.se/produkt/{}/{}-{}",
                    self.category_level1.to_lowercase(),
                    self.product_name_bold.to_lowercase().replace(' ', "-"),
                    self.product_number,
                )
            )
        )
    }

    #[must_use]
    pub fn link_display_name(&self) -> String {
        format!(
            "{}",
            link(
                &self.product_name_bold,
                format!(
                    "https://systembolaget.se/produkt/{}/{}-{}",
                    self.category_level1.to_lowercase(),
                    self.product_name_bold.to_lowercase().replace(' ', "-"),
                    self.product_number,
                )
            )
        )
    }

    #[must_use]
    pub fn link_display_volume(&self) -> String {
        format!(
            "{}",
            link(
                format!("{}: {} ml", self.product_name_bold, self.volume),
                format!(
                    "https://systembolaget.se/produkt/{}/{}-{}",
                    self.category_level1.to_lowercase(),
                    self.product_name_bold.to_lowercase().replace(' ', "-"),
                    self.product_number,
                )
            )
        )
    }

    #[must_use]
    pub fn link_display_vintage(&self) -> String {
        format!(
            "{}",
            link(
                format!(
                    "{}, {}",
                    self.product_name_bold,
                    self.vintage.clone().unwrap_or_default()
                ),
                format!(
                    "https://systembolaget.se/produkt/{}/{}-{}",
                    self.category_level1.to_lowercase(),
                    self.product_name_bold.to_lowercase().replace(' ', "-"),
                    self.product_number,
                )
            )
        )
    }

    #[must_use]
    pub fn link_display_product_launch_date(&self) -> String {
        format!(
            "{}",
            link(
                format!("{}, {}", self.product_name_bold, self.product_launch_date),
                format!(
                    "https://systembolaget.se/produkt/{}/{}-{}",
                    self.category_level1.to_lowercase(),
                    self.product_name_bold.to_lowercase().replace(' ', "-"),
                    self.product_number,
                )
            )
        )
    }

    #[must_use]
    pub fn link_display_ml_per_crown(&self) -> String {
        let url = format!(
            "https://systembolaget.se/produkt/{}/{}-{}",
            self.category_level1.to_lowercase(),
            self.product_name_bold.to_lowercase().replace(' ', "-"),
            self.product_number,
        );
        link(
            format!(
                "{} {}: {} ml/kr",
                self.product_name_bold,
                self.product_name_thin.clone().unwrap_or_default(),
                self.to_ml_per_crown(),
            ),
            &url,
        )
        .to_string()
    }

    #[must_use]
    pub fn link_display_crowns_per_ml(&self) -> String {
        format!(
            "{}",
            link(
                format!(
                    "{}: {} kr/ml",
                    self.product_name_bold,
                    self.to_crowns_per_ml()
                ),
                format!(
                    "https://systembolaget.se/produkt/{}/{}-{}",
                    self.category_level1.to_lowercase(),
                    self.product_name_bold.to_lowercase().replace(' ', "-"),
                    self.product_number,
                )
            )
        )
    }
}

impl IntoIterator for Assortment {
    type Item = (String, Product);
    type IntoIter = std::vec::IntoIter<(String, Product)>;

    fn into_iter(self) -> Self::IntoIter {
        self.products
            .into_iter()
            .map(|product| (product.product_name_bold.clone(), product))
            .collect::<Vec<(String, Product)>>()
            .into_iter()
    }
}

impl From<Assortment> for Vec<(String, Product)> {
    fn from(assortment: Assortment) -> Self {
        assortment
            .products
            .into_iter()
            .map(|product| (product.product_number.clone(), product))
            .collect()
    }
}

impl<S: BuildHasher + Default> From<Assortment> for HashMap<String, Product, S> {
    fn from(assortment: Assortment) -> Self {
        Self::from_iter(Into::<Vec<(String, Product)>>::into(assortment))
    }
}
