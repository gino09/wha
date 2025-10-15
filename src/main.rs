use actix_web::{App, HttpServer, Responder};

#[actix_web::get("/greet")]
async fn greet() -> impl Responder {
    format!("hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Starting server on port {port}");

    HttpServer::new(|| App::new().service(greet))
        .bind(("0.0.0.0", port))?
        .workers(2)
        .run()
        .await
}
