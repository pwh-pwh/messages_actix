use std::env;
use messages_actix::MessageApp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let app = MessageApp::new(8081);
    app.run().await
}
