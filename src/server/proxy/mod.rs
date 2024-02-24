// use actix_proxy::{IntoHttpResponse, SendRequestError};
// use actix_web::{get, web, HttpResponse};
// use awc::Client;

// #[get("/{url:.*}")]
// async fn proxy(
//     path: web::Path<(String,)>,
//     client: web::Data<Client>,
// ) -> Result<HttpResponse, SendRequestError> {
//     let (url,) = path.into_inner();

//     let url = format!("https://duckduckgo.com/{url}");

//     // here we use `IntoHttpResponse` to return the request to
//     // duckduckgo back to the client that called this endpoint
//     Ok(client.get(&url).send().await?.into_http_response())
// }
