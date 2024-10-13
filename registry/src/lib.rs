use adapter::{
    database::ConnectionPool,
    redis::RedisClient,
    repository::{
        auth::AuthRepositoryImpl, book::BookRepositoryImpl, checkout::CheckoutRepositoryImpl,
        health::HealthCheckRepositoryImpl, user::UserRepositoryImpl,
    },
};
use kernel::repository::{
    auth::AuthRepository, book::BookRepository, checkout::CheckoutRepository,
    health::HealthCheckRepository, user::UserRepository,
};
use shared::config::AppConfig;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppRegistryImpl {
    pub health_check_repository: Arc<dyn HealthCheckRepository>,
    pub book_repository: Arc<dyn BookRepository>,
    pub auth_repository: Arc<dyn AuthRepository>,
    pub user_repository: Arc<dyn UserRepository>,
    pub checkout_repository: Arc<dyn CheckoutRepository>,
}

impl AppRegistryImpl {
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
        let user_repository = Arc::new(UserRepositoryImpl::new(pool.clone()));
        let checkout_repository = Arc::new(CheckoutRepositoryImpl::new(pool.clone()));

        Self {
            health_check_repository,
            book_repository,
            auth_repository,
            user_repository,
            checkout_repository,
        }
    }

    // pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
    //     self.health_check_repository.clone()
    // }

    // // FIXME: 使っていない？
    // pub fn book_repository(&self) -> Arc<dyn BookRepository> {
    //     self.book_repository.clone()
    // }

    // pub fn auth_repository(&self) -> Arc<dyn AuthRepository> {
    //     self.auth_repository.clone()
    // }

    // pub fn user_repository(&self) -> Arc<dyn UserRepository> {
    //     self.user_repository.clone()
    // }

    // pub fn checkout_repository(&self) -> Arc<dyn CheckoutRepository> {
    //     self.checkout_repository.clone()
    // }
}

#[mockall::automock]
pub trait AppRegistryExt {
    fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository>;
    fn book_repository(&self) -> Arc<dyn BookRepository>;
    fn auth_repository(&self) -> Arc<dyn AuthRepository>;
    fn user_repository(&self) -> Arc<dyn UserRepository>;
    fn checkout_repository(&self) -> Arc<dyn CheckoutRepository>;
}

impl AppRegistryExt for AppRegistryImpl {
    fn book_repository(&self) -> Arc<dyn BookRepository> {
        self.book_repository.clone()
    }

    fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }

    fn auth_repository(&self) -> Arc<dyn AuthRepository> {
        self.auth_repository.clone()
    }

    fn user_repository(&self) -> Arc<dyn UserRepository> {
        self.user_repository.clone()
    }

    fn checkout_repository(&self) -> Arc<dyn CheckoutRepository> {
        self.checkout_repository.clone()
    }
}

pub type AppRegistry = Arc<dyn AppRegistryExt + Send + Sync + 'static>;
