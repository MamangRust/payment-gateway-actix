use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::{MigrationTrait, SchemaManager};

use crate::utils::errors::ConnectionManagerError;

pub struct ConnectionManager;

impl ConnectionManager {
    pub async fn new_pool<M: MigrationTrait + Default>(
        connection_string: &str,
        run_migrations: bool
    ) -> Result<DatabaseConnection, ConnectionManagerError> {
        let pool = Database::connect(connection_string).await
        .map_err(ConnectionManagerError::ConnectionError)?;
        

        if run_migrations {
            let schema_manager = SchemaManager::new(&pool);
            let migration = M::default();

               migration.up(&schema_manager).await
                .map_err(ConnectionManagerError::MigrationError)?;
        }
        
        Ok(pool)
    }
}