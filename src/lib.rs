use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct MessageApp {
    port: u16,
}

struct AppState {
    server_id: usize,
    request_count: Cell<usize>,
    messages: Arc<Mutex<Vec<String>>>,
}

#[derive(Serialize)]
struct IndexResponse {
    server_id: usize,
    request_count: usize,
    messages: Vec<String>,
}

#[derive(Deserialize)]
struct PostInput {
    message: String,
}

#[derive(Serialize)]
struct PostResponse {
    server_id: usize,
    request_count: usize,
    message: String,
}

#[get("/")]
async fn index(state: web::Data<AppState>) -> std::io::Result<impl Responder> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let ms = state.messages.lock().unwrap();
    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        messages: ms.clone(),
    }))
}

async fn post(
    state: web::Data<AppState>,
    input: web::Json<PostInput>,
) -> std::io::Result<impl Responder> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let mut ms = state.messages.lock().unwrap();
    ms.push(input.message.clone());
    Ok(web::Json(PostResponse {
        server_id: state.server_id,
        request_count,
        message: input.message.clone(),
    }))
}

#[post("/clear")]
async fn clear(state: web::Data<AppState>) -> std::io::Result<impl Responder> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let mut ms = state.messages.lock().unwrap();
    ms.clear();
    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        messages: vec![],
    }))
}

impl MessageApp {
    pub fn new(port: u16) -> Self {
        Self { port }
    }
    pub async fn run(&self) -> std::io::Result<()> {
        println!("Starting http server: 127.0.0.1:{}", self.port);
        let messages = Arc::new(Mutex::new(vec![]));
        HttpServer::new(move || {
            App::new()
                .app_data(Data::new(AppState {
                    server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
                    request_count: Cell::new(0),
                    messages: messages.clone(),
                }))
                .wrap(Logger::default())
                .service(index)
                .service(
                    web::resource("/send")
                        .app_data(web::JsonConfig::default().limit(4096))
                        .route(web::post().to(post)),
                )
                .service(clear)
        })
        .bind(("127.0.0.1", self.port))?
        .workers(4)
        .run()
        .await
    }
}
