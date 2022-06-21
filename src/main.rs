extern crate dotenv;
use dotenv::dotenv;
use std::env;

use std::sync::Mutex;
use sqlx;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};

use serde::Deserialize;


/// Task  
/// CREATE TABLE public.task (
///	description varchar NOT NULL,
///	priority int NOT NULL,
///	id int NOT NULL GENERATED ALWAYS AS IDENTITY
/// );
/// 
#[derive(Debug, Deserialize)]
struct Task {
    description: String,
}

async fn list_view(data: Data<Mutex<MyPool>>) -> impl Responder {
    let my_pool = &data.lock().unwrap();

    let tasks = sqlx::query_as!(Task,
        "
        SELECT description from public.task
        "
    )
    .fetch_all(&my_pool.pool) // -> Vec<Task>
    .await.unwrap();

    let output: String = tasks.iter().map(|x| x.description.to_owned() + ", ").collect();
   
    HttpResponse::Ok().body(format!("Tasks to do: {}", output))
}


// This handler is only called if:
// - request headers declare the content type as `application/x-www-form-urlencoded`
// - request payload is deserialized into a `Task` struct from the URL encoded format
async fn add_task_view(form: web::Form<Task>, data: Data<Mutex<MyPool>>) -> impl Responder {
    println!("Add task view: {}", &form.description);
    let my_pool = &data.lock().unwrap();
    let rec = sqlx::query!(
        r#"
INSERT INTO public.task ( description, priority )
VALUES ( $1, 1 )
RETURNING id
        "#,
        &form.description,
    )
    .fetch_one(&my_pool.pool)
    .await.unwrap();

    HttpResponse::Ok().body(format!("Task added: {}", rec.id))
}


struct MyPool {
     pool: sqlx::Pool<sqlx::Postgres>,
 }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await.unwrap();

    let pool_struct = MyPool{ pool };

    let data = Data::new(Mutex::new(pool_struct));

    HttpServer::new(move|| {
        App::new()
            .app_data(Data::clone(&data))
            .route("/", web::get().to(list_view))
            .route("/add", web::post().to(add_task_view))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await 
}