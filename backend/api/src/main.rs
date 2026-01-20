use actix_web::{web, App, HttpServer};
use std::env;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use actix_cors::Cors;

// Declara los módulos que vamos a usar
mod models;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    println!("✅ Database connection pool established.");
    println!("🚀 Server starting on http://127.0.0.1:8080");

    HttpServer::new(move || {
        // Configuración de CORS. En desarrollo, permitimos todo.
        // En producción, serías más restrictivo (ej. .allowed_origin("https://tu-dominio.com"))
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(handlers::health_check))
            .service(
                web::scope("/api") // Agrupa endpoints bajo /api
                    .route("/actors", web::get().to(handlers::get_actors))
                    // Aquí añadiremos la ruta para crear rentas
                    .route("/rentals", web::post().to(handlers::create_rental))
                    // La ruta de borrado va aquí
                    .route("/rentals/{id}", web::delete().to(handlers::cancel_rental))
                    // Añade la ruta para la devolución. {id} es un parámetro de ruta.
                    .route("/rentals/{id}/return", web::put().to(handlers::return_dvd))
                    // Ruta para el reporte de ingresos por staff
                    .route("/staff/revenue", web::get().to(handlers::get_staff_revenue))
                    // Ruta para el reporte de películas más rentadas
                    .route("/films/most-rented", web::get().to(handlers::get_most_rented_films))
                    // Ruta para el reporte de DVDs no devueltos
                    .route("/rentals/overdue", web::get().to(handlers::get_overdue_rentals))
                    // Ruta para el reporte de rentas por cliente
                    .route("/customers/{id}/rentals", web::get().to(handlers::get_customer_rentals))
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}