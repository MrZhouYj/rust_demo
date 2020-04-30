extern crate diesel_demo;
use diesel_demo::db_connection;
#[macro_use]
extern crate redis_async;
use actix::prelude::*;
use actix_redis::RedisActor;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_demo::matcher::Matcher;
use diesel_demo::models::Order;
use serde_json::json;
use std::str::FromStr;
use std::{env, io};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let db_pool = db_connection::db_pool();

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    let mut mat = Matcher::default();
    let price = "100";
    let price_decimal = BigDecimal::from_str(&price).unwrap();
    let volume = "100";
    let volume_decimal = BigDecimal::from_str(&volume).unwrap();
    mat.add_order(Order {
        id: 10,
        member_id: 10,
        market_id: "btcusdt".to_string(),
        ask_or_bid: "bid".to_string(),
        state: 100,
        price: price_decimal,
        volume: volume_decimal,
    });

    HttpServer::new(move || {
        let redis_addr = RedisActor::start("127.0.0.1:6379");
        App::new()
            .data(redis_addr)
            .wrap(middleware::Logger::default())
            .data(db_pool.clone())
            .service(web::resource("/").route(web::get().to(|req: HttpRequest| {
                HttpResponse::Ok().json(json!({
                    "state": "success"
                }))
            })))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
