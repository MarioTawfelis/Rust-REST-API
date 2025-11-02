use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

/// Call once and pass pool around
pub fn init_pool(database_url: &str) -> Result<PgPool, r2d2::Error> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(15)
        .build(manager)
}

/// Borrow a connection from the pool (blocking!!)
pub fn get_conn(pool: &PgPool) -> Result<PooledConnection<ConnectionManager<PgConnection>>, r2d2::Error> {
    pool.get()
}

/// Run pending database migrations
pub fn run_migrations(conn: &mut PgConnection) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    conn.run_pending_migrations(MIGRATIONS).map(|_| ())
}


/// The below code is not my own///
/// Bridge sync Diesel calls onto a blocking thread for async runtimes (Warp/Tokio).
///
/// Usage:
/// ```ignore
/// let pool = pool.clone();
/// let user = with_conn(pool, |conn| repo::create_user(conn, &new_user))
///     .await
///     .map_err(AppError::from)?;
/// ```
/// This keeps blocking DB work off the async reactor.
pub async fn with_conn<F, T, E>(pool: PgPool, f: F) -> Result<T, E>
where
    F: FnOnce(&mut PgConnection) -> Result<T, E> + Send + 'static,
    T: Send + 'static,
    E: Send + 'static,
{
    tokio::task::spawn_blocking(move || {
        let mut conn = pool.get().expect("failed to get DB connection");
        f(&mut conn)
    })
    .await
    .expect("DB task panicked")
}
