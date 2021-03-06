use actix_web::{
    get,
    App,
    HttpServer,
    Responder,
};

#[get("/hello")]
async fn index() -> impl Responder {
    format!("Hello, world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
