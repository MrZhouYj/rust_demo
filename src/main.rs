extern crate diesel_demo;

use diesel_demo::{db_connection, models};
use diesel_demo::schema::orders::dsl::orders;
use actix_web::{HttpServer, App, middleware, web, HttpRequest, HttpResponse};
use std::{env, io};
use serde_json::json;


#[actix_rt::main]
async fn main() -> io::Result<()> {

    let db_pool = db_connection::db_pool();

    let results = orders.select();

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }

    //env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    HttpServer::new( move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(db_pool.clone())
            .service(
                web::resource("/").route(
                    web::get().to(|req: HttpRequest| {
                        HttpResponse::Ok().json(json!({
                            "state": "success"
                        }))
                    })
                )
            )
     })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

