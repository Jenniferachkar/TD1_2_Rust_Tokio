use tokio::time::{sleep, Duration};
use rand::Rng;
use std::time::Instant;
use sqlx::{postgres::PgPoolOptions, PgPool};
use chrono::Utc;
use serde::Deserialize;
use dotenvy::dotenv;
use std::env;
use tracing::{info, error};
use tokio::{signal, time::interval};

#[derive(Debug, Clone)]
struct StockPrice {
    symbol: String,
    price: f64,
    source: String,
    timestamp: i64,
}

// Mock async fetching

async fn fetch_mock_price(symbol: &str) -> f64 {
    sleep(Duration::from_millis(500)).await;
    let mut rng = rand::thread_rng();
    let price: f64 = rng.gen_range(100.0..200.0);
    println!("the price for {} is ${:.2}", symbol, price);
    price
}

//Finnhub

#[derive(Deserialize)]
struct FinnhubResponse {
    c: f64,
}

async fn fetch_finnhub(symbol: &str) -> Result<StockPrice, Box<dyn std::error::Error>> {
    dotenv().ok();
    let key = env::var("FINNHUB_KEY").unwrap_or_else(|_| "demo".to_string());
    let url = format!("https://finnhub.io/api/v1/quote?symbol={}&token={}", symbol, key);

    let resp = reqwest::get(&url).await?.json::<FinnhubResponse>().await?;
    Ok(StockPrice {
        symbol: symbol.to_string(),
        price: resp.c,
        source: "finnhub".to_string(),
        timestamp: Utc::now().timestamp(),
    })
}

// PostgreSQL

async fn save_price(pool: &PgPool, price: &StockPrice) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO stock_prices (symbol, price, source, timestamp)
        VALUES ($1, $2, $3, $4)
        "#,
        price.symbol,
        price.price,
        price.source,
        price.timestamp
    )
    .execute(pool)
    .await?;
    Ok(())
}

//Cycle Periodique

async fn fetch_and_save_all(pool: &PgPool, symbols: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting fetch cycle for {} symbols", symbols.len());

    for symbol in symbols {
        let (mock_price, finnhub_result) = tokio::join!(
            fetch_mock_price(symbol),
            fetch_finnhub(symbol)
        );

        if let Ok(price) = finnhub_result {
            if let Err(e) = save_price(pool, &price).await {
                error!("Failed to save finnhub price: {}", e);
            }
        }

        println!("Mock price for {}: ${:.2}", symbol, mock_price);
    }

    info!("Completed fetch cycle");
    Ok(())
}

// Main TOKIO

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt().with_env_filter("info").init();

    info!("Starting stock price aggregator...");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().max_connections(5).connect(&db_url).await?;

    let symbols = vec!["AAPL".to_string(), "GOOGL".to_string(), "MSFT".to_string()];
    let mut fetch_interval = interval(Duration::from_secs(60));

    loop {
        tokio::select! {
            _ = fetch_interval.tick() => {
                if let Err(e) = fetch_and_save_all(&pool, &symbols).await {
                    error!("Error during fetch cycle: {}", e);
                }
            }
            _ = signal::ctrl_c() => {
                info!("Shutdown signal received");
                break;
            }
        }
    }

    pool.close().await;
    info!("Shutdown complete");
    Ok(())
}
