use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use actix_web::web::Query;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Calendar {
    name: String,
    description: String,
    start: String,
    end: String,
}

const PIG_COUNT: i32 = 28;
// const TAU: f32 = 6.28;

#[get("/")]
async fn hello() -> impl Responder {
    println!("/ was visited");
    HttpResponse::Ok().body("Hello world!")
}

#[get("/goats")]
async fn goats() -> impl Responder {
    println!("/goats was visited");
    HttpResponse::Ok().body("you have found the goats!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("/echo was visited");
    HttpResponse::Ok().body(req_body)
}

#[get("/calendar")]
async fn calendar(query: Query<Calendar>) -> impl Responder {
    let calendar = query.into_inner();
    println!("/calendar was visited: {:?}", calendar);
    HttpResponse::Ok().body(serde_json::to_string(&calendar).unwrap())
}

async fn manual_hello() -> impl Responder {
    println!("/hey was visited");
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut goat_count: i64 = 5;
    let apple_count = 5;
    println!("Hello, world!");
    goat_count += 2000000000;
    println!("we have {goat_count} goats and {apple_count} apples and {PIG_COUNT} pigs");
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(hello)
            .service(calendar)
            .service(goats)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
