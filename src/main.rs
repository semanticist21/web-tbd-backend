pub mod server;

use actix_cors::Cors;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let base_addr = "127.0.0.1";
    let port = "8080";

    HttpServer::new(move || {
        let allowed = vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"];

        let cors = Cors::default()
            .allowed_origin([base_addr, port].join(":").as_str())
            .allowed_methods(allowed);

        App::new().wrap(cors).service(server::hello)
    })
    .bind([base_addr, port].join(":").as_str())?
    .run()
    .await
}
