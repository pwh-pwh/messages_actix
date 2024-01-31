use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::http::header;
use actix_web::middleware::Logger;
use serde::Serialize;

pub struct MessageApp {
    port: u16,
}

#[derive(Serialize)]
struct IndexResponse {
    message: String,
}


#[get("/")]
async fn index(req: HttpRequest) -> std::io::Result<impl Responder> {
    let hello = req
        .headers()
        .get("hello")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_else(|| "world");
    Ok(web::Json(
        IndexResponse {
            message: hello.to_owned(),
        }
    ))
}

impl MessageApp {
    pub fn new(port: u16) -> Self {
        Self {
            port
        }
    }
    pub async fn run(&self) -> std::io::Result<()> {
        println!("Starting http server: 127.0.0.1:{}", self.port);
        HttpServer::new(|| {
            App::new()
                .wrap(Logger::default())
                .service(index)
        })
            .bind(("127.0.0.1", self.port))?
            .workers(8)
            .run()
            .await
    }
}