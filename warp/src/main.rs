#![feature(try_blocks)]
use anyhow::{format_err, Context};
use refinery::config::ConfigDbType;
use sqlx::PgPool;
use std::{env, process};
use tracing::{debug, error, info};
use url::Url;
use warp::Filter;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        error!("{:?}", e);
        process::exit(1);
    }
}

mod api;
mod footballer;
mod repository;

async fn run() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "warn")
    };
    tracing_subscriber::fmt::init();

    let config = Config::new()?;

    migrate(config.db_connection_url.clone()).await?;

    let pool = PgPool::builder()
        .max_size(config.db_pool_max_size)
        .min_size(config.db_pool_min_size)
        .build(&config.db_connection_url)
        .await?;

    let api = api::get_footballer(pool.clone())
        .or(api::search_footballers(pool.clone()))
        .or(api::create_footballer(pool.clone()))
        .or(api::delete_footballer(pool));

    warp::serve(api).run(([0, 0, 0, 0], 8080)).await;

    Ok(())
}

async fn migrate(url: String) -> anyhow::Result<()> {
    tokio::task::spawn_blocking(move || {
        let parsed_url = Url::parse(url.as_str())?;

        let mut connection = refinery::config::Config::new(ConfigDbType::Postgres)
            .set_db_host(parsed_url.host_str().unwrap())
            .set_db_name(
                &parsed_url
                    .path_segments()
                    .expect("Database name path")
                    .next()
                    .unwrap(),
            )
            .set_db_pass(parsed_url.password().unwrap())
            .set_db_port(format!("{}", parsed_url.port().unwrap()).as_str())
            .set_db_user(parsed_url.username());
        debug!("Migrating against {:?} (Config: {:?})", url, connection);

        info!("Migrate Database");
        embedded::migrations::runner().run(&mut connection)?;
        Ok(())
    })
    .await?
}

struct Config {
    db_connection_url: String,
    db_pool_max_size: u32,
    db_pool_min_size: u32,
}

impl Config {
    fn new() -> anyhow::Result<Self> {
        let db_connection_url = env("DATABASE_URL")?;
        let db_pool_max_size = match env("DATABASE_POOL_MAX_SIZE") {
            Ok(it) => it.parse().context("Parsing DATABASE_POOL_MAX_SIZE")?,
            Err(_) => 15,
        };
        let db_pool_min_size = match env("DATABASE_POOL_MIN_SIZE") {
            Ok(it) => it.parse().context("Parsing DATABASE_POOL_MIN_SIZE")?,
            Err(_) => 5,
        };
        Ok(Self {
            db_connection_url,
            db_pool_max_size,
            db_pool_min_size,
        })
    }
}

fn env(key: &str) -> anyhow::Result<String> {
    env::var(key).map_err(|_| format_err!("Environment variable {:?} not set", key))
}

mod embedded {
    use refinery::embed_migrations;

    embed_migrations!();
}
