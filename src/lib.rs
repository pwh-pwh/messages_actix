use std::cell::Cell;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use serde::Serialize;

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct MessageApp {
    port: u16,
}

struct AppState {
    server_id: usize,
    request_count: Cell<usize>,
    messages: Arc<Mutex<Vec<String>>>
}

#[derive(Serialize)]
struct IndexResponse {
    server_id: usize,
    request_count: usize,
    messages: Vec<String>
}


#[get("/")]
async fn index(state: web::Data<AppState>) -> std::io::Result<impl Responder> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let ms = state.messages.lock().unwrap();
    Ok(web::Json(
        IndexResponse {
            server_id: state.server_id,
            request_count,
            messages: ms.clone()
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
        let messages = Arc::new(Mutex::new(vec![]));
        HttpServer::new(move|| {
            App::new()
                .app_data(
                    Data::new(
                        AppState {
                            server_id: SERVER_COUNTER.fetch_add(1,Ordering::SeqCst),
                            request_count: Cell::new(0),
                            messages: messages.clone(),
                        }
                    )
                )
                .wrap(Logger::default())
                .service(index)
        })
            .bind(("127.0.0.1", self.port))?
            .workers(4)
            .run()
            .await
    }
}