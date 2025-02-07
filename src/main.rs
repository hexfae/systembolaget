use askama_axum::{IntoResponse, Response};
use axum::{Router, extract::State, routing::get};
use systembolaget::{error::Result, systembolaget::Assortment};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt().init();
    create_app().await?;
    Ok(())
}

async fn create_app() -> Result<()> {
    let assortment = serde_json::from_str::<Assortment>(include_str!("../assortment.json"))?;
    let app = Router::new()
        .route("/", get(index))
        .route("/api/assortment/page/:index", get(page_index))
        .with_state(assortment);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    Ok(axum::serve(listener, app).await?)
}

async fn index(State(assortment): State<Assortment>) -> Response {
    assortment.into_response()
}

async fn page_index(State(assortment): State<Assortment>) -> Response {
    let products = assortment
        .products
        .get(0..=20)
        .unwrap_or_else(|| &Vec::new());
    todo!();
}
