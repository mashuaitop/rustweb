use actix_web::{web, App, HttpResponse, HttpServer, Responder, Error, error,};

use actix_cors::Cors;

use sqlx::MySqlPool;

mod structs;


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn get_users(pool: web::Data<MySqlPool>) -> Result<HttpResponse, Error> {
    let recs = sqlx::query_as!(
        structs::Book,
        r#"
        SELECT * FROM book
        "#
    )
    .fetch_all(pool.as_ref()) 
    .await 
    .map_err(|e| { 
        eprintln!("Failed to fetch users: {}", e);
        error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().json(recs))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = "mysql://root:123456@localhost:3306/book";

    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to MySQL.");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
        .wrap(cors)
        .data(pool.clone())
        .service(
            web::scope("/web")
                .route("/hello", web::get().to(manual_hello))
                .route("/book/findall", web::get().to(get_users))
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
