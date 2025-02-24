use askama_axum::{IntoResponse, Response, Template};
use axum::{
    Router,
    extract::{Path, State},
    http::HeaderName,
    routing::get,
};
use reqwest::header::CONTENT_TYPE;
use systembolaget::{
    error::Result,
    systembolaget::{Assortment, Product},
};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt().init();
    create_app().await?;
    Ok(())
}

async fn create_app() -> Result<()> {
    println!("wait");
    let assortment = serde_json::from_str::<Assortment>(include_str!("../assortment.json"))?;
    let app = Router::new()
        .route("/", get(index))
        .route("/assortment/page/{index}", get(page_index))
        .route("/api/assortment/page/{index}", get(htmx_page_index))
        .route("/style.css", get(style))
        .with_state(assortment);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("ready");
    Ok(axum::serve(listener, app).await?)
}

const STYLE: &str = include_str!("../templates/style.css");

async fn style() -> ([(HeaderName, &'static str); 1], &'static str) {
    ([(CONTENT_TYPE, "text/css")], STYLE)
}

async fn index(State(mut assortment): State<Assortment>) -> Response {
    assortment.products = assortment.products.get(0..=19).unwrap_or(&[]).to_vec();
    assortment.into_response()
}

async fn page_index(index: Path<usize>) -> Response {
    AssortmentPage {
        current_page: *index,
    }
    .into_response()
}

async fn htmx_page_index(State(assortment): State<Assortment>, index: Path<usize>) -> Response {
    let products = assortment
        .products
        .get(
            index.saturating_sub(1).saturating_mul(20)..=index.saturating_mul(20).saturating_sub(1),
        )
        .unwrap_or(&[])
        .to_vec();
    let products = HtmxProducts {
        products,
        doc_count: assortment.metadata.doc_count,
        previous_page: index.wrapping_sub(1),
        next_page: index.wrapping_add(1),
    };
    products.into_response()
}

#[derive(Template)]
#[template(path = "assortment_page.html")]
struct AssortmentPage {
    current_page: usize,
}

#[derive(Template)]
#[template(path = "htmx/htmx_assortment_page.html")]
struct HtmxProducts {
    products: Vec<Product>,
    doc_count: u16,
    previous_page: usize,
    next_page: usize,
}
