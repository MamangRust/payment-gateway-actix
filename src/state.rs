use sea_orm::DatabaseConnection;

use crate::{config::{hashing::Hashing, jwt_config::JwtConfig}, utils::di::DependenciesInject};



#[derive(Clone)]
pub struct AppState{
    pub di_container: DependenciesInject,
    pub jwt_config: JwtConfig,
}

impl AppState{
    pub fn new(pool: DatabaseConnection, jwt_secret: &str) -> Self{
        let jwt_config = JwtConfig::new(jwt_secret);
        let hashing = Hashing::new();

        let di_container = DependenciesInject::new(pool, hashing, jwt_config.clone());

        Self { di_container, jwt_config }
    }
}