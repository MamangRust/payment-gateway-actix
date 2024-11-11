use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web::Data, App, HttpServer};
use dotenv::dotenv;
use example_payment_gateway::{config::{config::Config, database::ConnectionManager}, handler::router_config, migration::m20220101_000001_create_table::Migration, state::AppState};
use example_payment_gateway::utils::log_tracing;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // if std::env::var_os("RUST_LOG").is_none() {
    //     std::env::set_var("RUST_LOG", "actix_web=info");
    // }


    // if std::env::var_os("RUST_LOG").is_none() {
    //     std::env::set_var("RUST_LOG", "actix_web=info");
    // }

    dotenv().ok();
    // env_logger::init();

    log_tracing::tracing();

    let config = Config::init();

    let db_pool = ConnectionManager::new_pool::<Migration>(&config.database_url, config.run_migrations).await?;

    let port = config.port;

    let state = AppState::new(db_pool, &config.jwt_secret);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .configure(router_config)
            .app_data(Data::new(state.clone()))
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
    .expect("Failed");

    Ok(())
}
