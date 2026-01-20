use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::{Actor, CreateRental, RentalResponse, CustomerRental, OverdueRental, MostRentedFilm, StaffRevenue};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("API is running!")
}

pub async fn get_actors(pool: web::Data<PgPool>) -> impl Responder {
    // ... el código de la función se mantiene igual
    let query = "SELECT actor_id, first_name, last_name FROM actor ORDER BY actor_id LIMIT 10;";
    match sqlx::query_as::<_, Actor>(query)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(actors) => HttpResponse::Ok().json(actors),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch actors."),
    }
}

// Handler para la ruta POST /rentals
pub async fn create_rental(
    pool: web::Data<PgPool>,
    rental_data: web::Json<CreateRental>,
) -> impl Responder {
    let query = "
        INSERT INTO rental (inventory_id, customer_id, staff_id, rental_date, last_update)
        VALUES ($1, $2, $3, NOW(), NOW())
        RETURNING rental_id, rental_date;
    ";

    match sqlx::query_as::<_, RentalResponse>(query)
        .bind(rental_data.inventory_id)
        .bind(rental_data.customer_id)
        .bind(rental_data.staff_id)
        .fetch_one(pool.get_ref()) // fetch_one porque esperamos que devuelva una sola fila
        .await
    {
        Ok(new_rental) => HttpResponse::Created().json(new_rental), // 201 Created
        Err(e) => {
            // Imprime el error en la consola del servidor para depuración
            eprintln!("Failed to create rental: {}", e);
            HttpResponse::InternalServerError().body("Failed to create rental.")
        }
    }
}

// Handler para la ruta PUT /rentals/{id}/return
pub async fn return_dvd(
    pool: web::Data<PgPool>,
    path: web::Path<i32>, // Extrae el ID desde la URL
) -> impl Responder {
    let rental_id = path.into_inner();

    // Esta consulta actualiza la fecha de devolución a la hora actual
    let query = "
        UPDATE rental
        SET return_date = NOW()
        WHERE rental_id = $1
        RETURNING rental_id, rental_date, return_date;
    ";

    // NOTA: Necesitaremos una nueva struct para la respuesta que incluya return_date.
    // Por ahora, solo confirmaremos que la operación se realizó.
    // En un paso posterior, crearemos la struct de respuesta.
    match sqlx::query(query)
        .bind(rental_id)
        .execute(pool.get_ref())
        .await
    {
        Ok(result) => {
            // execute() devuelve un resumen del resultado, no los datos.
            // Verificamos si alguna fila fue afectada. Si no, no se encontró el ID.
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().body(format!("Rental with ID {} not found", rental_id))
            } else {
                HttpResponse::Ok().body(format!("Rental {} returned successfully", rental_id))
            }
        }
        Err(e) => {
            eprintln!("Failed to return DVD: {}", e);
            HttpResponse::InternalServerError().body("Failed to return DVD.")
        }
    }
}

// Handler para la ruta DELETE /rentals/{id}
pub async fn cancel_rental(
    pool: web::Data<PgPool>,
    path: web::Path<i32>, // Extrae el ID desde la URL
) -> impl Responder {
    let rental_id = path.into_inner();

    // Lógica de negocio: En un sistema real, aquí podrías verificar
    // si la renta ya fue devuelta antes de permitir el borrado.
    // Por ahora, procederemos con el borrado directo.

    let query = "DELETE FROM rental WHERE rental_id = $1;";

    match sqlx::query(query)
        .bind(rental_id)
        .execute(pool.get_ref())
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().body(format!("Rental with ID {} not found", rental_id))
            } else {
                // El código de estado 204 No Content es la respuesta estándar
                // para un DELETE exitoso. No lleva cuerpo de respuesta.
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => {
            eprintln!("Failed to cancel rental: {}", e);
            HttpResponse::InternalServerError().body("Failed to cancel rental.")
        }
    }
}

// Handler para la ruta GET /customers/{id}/rentals
pub async fn get_customer_rentals(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let customer_id = path.into_inner();

    // Esta consulta une las tablas rental, inventory y film para obtener el título.
    let query = "
        SELECT
            r.rental_id,
            f.title as film_title,
            r.rental_date,
            r.return_date
        FROM rental r
        JOIN inventory i ON r.inventory_id = i.inventory_id
        JOIN film f ON i.film_id = f.film_id
        WHERE r.customer_id = $1
        ORDER BY r.rental_date DESC;
    ";

    match sqlx::query_as::<_, CustomerRental>(query)
        .bind(customer_id)
        .fetch_all(pool.get_ref()) // fetch_all porque esperamos múltiples filas
        .await
    {
        Ok(rentals) => HttpResponse::Ok().json(rentals),
        Err(e) => {
            eprintln!("Failed to fetch customer rentals: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch customer rentals.")
        }
    }
}

// Handler para la ruta GET /rentals/overdue
pub async fn get_overdue_rentals(pool: web::Data<PgPool>) -> impl Responder {
    // Esta consulta une 4 tablas para obtener toda la información necesaria.
    // La condición clave es "WHERE r.return_date IS NULL".
    let query = "
        SELECT
            r.rental_id,
            f.title AS film_title,
            c.first_name || ' ' || c.last_name AS customer_name,
            r.rental_date
        FROM rental r
        JOIN inventory i ON r.inventory_id = i.inventory_id
        JOIN film f ON i.film_id = f.film_id
        JOIN customer c ON r.customer_id = c.customer_id
        WHERE r.return_date IS NULL
        ORDER BY r.rental_date ASC; -- Muestra las más antiguas primero
    ";

    match sqlx::query_as::<_, OverdueRental>(query)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(rentals) => HttpResponse::Ok().json(rentals),
        Err(e) => {
            eprintln!("Failed to fetch overdue rentals: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch overdue rentals.")
        }
    }
}

// Handler para la ruta GET /films/most-rented
pub async fn get_most_rented_films(pool: web::Data<PgPool>) -> impl Responder {
    // Esta consulta cuenta las rentas por película y las ordena
    // de mayor a menor, mostrando el TOP 10.
    let query = "
        SELECT
            f.title AS film_title,
            COUNT(r.rental_id) AS rental_count
        FROM rental r
        JOIN inventory i ON r.inventory_id = i.inventory_id
        JOIN film f ON i.film_id = f.film_id
        GROUP BY f.title
        ORDER BY rental_count DESC
        LIMIT 10;
    ";

    match sqlx::query_as::<_, MostRentedFilm>(query)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => {
            eprintln!("Failed to fetch most rented films: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch most rented films.")
        }
    }
}

// Handler para la ruta GET /staff/revenue
pub async fn get_staff_revenue(pool: web::Data<PgPool>) -> impl Responder {
    // Esta consulta suma todos los pagos gestionados por cada miembro del staff.
    let query = "
        SELECT
            s.first_name || ' ' || s.last_name AS staff_name,
            SUM(p.amount) AS total_revenue
        FROM payment p
        JOIN staff s ON p.staff_id = s.staff_id
        GROUP BY staff_name
        ORDER BY total_revenue DESC;
    ";

    match sqlx::query_as::<_, StaffRevenue>(query)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(revenue) => HttpResponse::Ok().json(revenue),
        Err(e) => {
            eprintln!("Failed to fetch staff revenue: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch staff revenue.")
        }
    }
}