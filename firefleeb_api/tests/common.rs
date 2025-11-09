use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use testcontainers::{GenericImage, RunnableImage, clients::Cli};

pub struct TestDb {
    pub pool: Pool<ConnectionManager<PgConnection>>,
    _container: testcontainers::Container<'static, GenericImage>, // keep it alive
}

pub fn setup_postgres() -> TestDb {
    // Start docker client
    let docker = Box::leak(Box::new(Cli::default())); // 'static for test lifetime

    // Postgres image with deterministic creds
    let image = RunnableImage::from(
        GenericImage::new("postgres", "15-alpine")
            .with_env_var("POSTGRES_USER", "postgres")
            .with_env_var("POSTGRES_PASSWORD", "password")
            .with_env_var("POSTGRES_DB", "testdb")
            .with_exposed_port(5432),
    );

    let container = docker.run(image);

    // Resolve host port mapped to container's 5432
    let host_port = container.get_host_port_ipv4(5432);
    let db_url = format!(
        "postgres://postgres:password@127.0.0.1:{}/testdb",
        host_port
    );

    // Build pool using your db module
    let pool = firefleeb_api::db::init_pool(&db_url).expect("pool");
    {
        // Run embedded migrations once
        let mut conn = firefleeb_api::db::get_conn(&pool).expect("conn");
        firefleeb_api::db::run_migrations(&mut conn).expect("migrations");
    }

    TestDb {
        pool,
        _container: container,
    }
}
