use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 8080;
    println!("Starting server on port {port}");

    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", port))?
        .workers(2)
        .run()
        .await
}