pub mod server;

use actix_cors::Cors;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let base_addr = "127.0.0.1";

    let bk_port = "443";
    let front_port = "8080";

    HttpServer::new(|| {
        let allowed = vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"];

        let cors = Cors::default()
            .allowed_origin([base_addr, backend_port].join(":").as_str())
            .allowed_methods(allowed);

        App::new().wrap(cors).service(server::hello)
        // .service(server::proxy)
    })
    .bind([base_addr, backend_port].join(":").as_str())?
    .run()
    .await
}
