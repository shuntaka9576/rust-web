use adapter::{
    database::ConnectionPool,
    redis::RedisClient,
    repository::{
        auth::AuthRepositoryImpl, book::BookRepositoryImpl, health::HealthCheckRepositoryImpl,
    },
};
use kernel::repository::{
    auth::AuthRepository, book::BookRepository, health::HealthCheckRepository,
};
use shared::config::AppConfig;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppRegistry {
    pub health_check_repository: Arc<dyn HealthCheckRepository>,
    pub book_repository: Arc<dyn BookRepository>,
    pub auth_repository: Arc<dyn AuthRepository>,
}

impl AppRegistry {
    pub fn new(
        pool: ConnectionPool,
        redis_client: Arc<RedisClient>,
        app_config: AppConfig,
    ) -> Self {
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));
        let book_repository = Arc::new(BookRepositoryImpl::new(pool.clone()));
        let auth_repository = Arc::new(AuthRepositoryImpl::new(
            pool.clone(),
            redis_client.clone(),
            app_config.auth.ttl,
        ));

        Self {
            health_check_repository,
            book_repository,
            auth_repository,
        }
    }

    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository().clone()
    }

    pub fn auth_repository(&self) -> Arc<dyn AuthRepository> {
        self.auth_repository.clone()
    }
}
