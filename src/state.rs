use std::path::Path;

use sqlx::SqlitePool;

use crate::configuration::Configuration;

#[derive(Clone)]
pub struct State {
    pool: SqlitePool,
}

impl State {
    pub fn new(configuration: &Configuration) -> Self {
        let mut pool: Option<SqlitePool> = None;
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let inner_pool = SqlitePool::connect(configuration.database_url())
                    .await
                    .unwrap_or_else(|_| {
                        panic!(
                            "Failed to create SQLite Pool: {}",
                            configuration.database_url()
                        )
                    });
                let migrator =
                    sqlx::migrate::Migrator::new(Path::new(configuration.migrations_path()))
                        .await
                        .expect("Failed create migrator");
                migrator
                    .run(&inner_pool)
                    .await
                    .expect("Failed to run migrations");
                pool = Some(inner_pool);
            });
        });

        let pool = pool.unwrap();
        Self { pool }
    }

    pub fn pool(&self) -> SqlitePool {
        self.pool.clone()
    }
}
