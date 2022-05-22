use crate::routes::{health_check, subscribe};

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_conn_pool: PgPool) -> Result<Server, std::io::Error> {
    //
    // Preparing the database connection pool as part of the *application state*.
    // This is used in the closure below, used by each request serving thread,
    // that's why a cloned reference (an `Arc`, included in `web::Data`) is provided.
    let db_conn_pool = web::Data::new(db_conn_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_conn_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
