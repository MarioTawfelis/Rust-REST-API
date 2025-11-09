use std::net::SocketAddr;

use dotenv::dotenv;
use firefleeb_api::db::{get_conn, init_pool, run_migrations, PgPool};
use firefleeb_api::routes::{
    cart_routes::cart_routes, handle_rejection, product_routes::product_routes,
    user_routes::user_routes,
};
use tracing_subscriber::EnvFilter;
use warp::Filter;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("server failed: {err}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    init_tracing();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set to start the API");
    let port = std::env::var("PORT")
        .ok()
        .and_then(|raw| raw.parse::<u16>().ok())
        .unwrap_or(8080);

    let pool = init_pool(&database_url).expect("failed to create DB pool");
    run_pending_migrations(&pool);

    let api = product_routes(pool.clone())
        .or(cart_routes(pool.clone()))
        .or(user_routes(pool))
        .recover(handle_rejection);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🚀 FireFleeb API listening on http://{addr}");
    warp::serve(api).run(addr).await;
    Ok(())
}

fn run_pending_migrations(pool: &PgPool) {
    let mut conn = get_conn(pool).expect("failed to get DB connection for migrations");
    run_migrations(&mut conn).expect("migrations failed");
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .try_init();
}
