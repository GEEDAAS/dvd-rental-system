use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use sqlx::types::Decimal;

// Struct para representar un actor
#[derive(Serialize, FromRow)]
pub struct Actor {
    pub actor_id: i32,
    pub first_name: String,
    pub last_name: String,
}

// Struct para recibir los datos de una nueva renta
#[derive(Deserialize)]
pub struct CreateRental {
    pub customer_id: i32,
    pub inventory_id: i32,
    pub staff_id: i32,
}

// Struct para la respuesta JSON al crear una renta
#[derive(Serialize, FromRow)]
pub struct RentalResponse {
    pub rental_id: i32,
    pub rental_date: DateTime<Utc>,
}

// Struct para representar una renta en el historial de un cliente
#[derive(Serialize, FromRow)]
pub struct CustomerRental {
    pub rental_id: i32,
    pub film_title: String, // Este campo viene de la tabla 'film'
    pub rental_date: DateTime<Utc>,
    // Usamos Option<> porque return_date puede ser NULO (no devuelto)
    pub return_date: Option<DateTime<Utc>>,
}

// Struct para representar una renta activa (no devuelta)
#[derive(Serialize, FromRow)]
pub struct OverdueRental {
    pub rental_id: i32,
    pub film_title: String,
    pub customer_name: String, // Combinaremos nombre y apellido
    pub rental_date: DateTime<Utc>,
}

// Struct para el reporte de las películas más rentadas
#[derive(Serialize, FromRow)]
pub struct MostRentedFilm {
    pub film_title: String,
    // La función COUNT() de SQL devuelve un tipo grande (bigint),
    // que se mapea a i64 en Rust.
    pub rental_count: i64,
}

// Struct para el reporte de ingresos por staff
#[derive(Serialize, FromRow)]
pub struct StaffRevenue {
    pub staff_name: String,
    pub total_revenue: Decimal,
}