use std::sync::Arc;

use boilermaker_core::logging;
use boilermaker_web::{WebApp, WebAppState};

#[tokio::main]
#[tracing::instrument]
async fn main() {
    color_eyre::install().expect("Failed to set up error handling");
    let _ = logging::init_tracing(3);

    let app_state = Arc::new(
        WebAppState::new()
            .await
            .expect("[boilermaker_web::main] Cannot create AppState"),
    );

    let listen_addr = "0.0.0.0:8000";

    let app = WebApp::build(listen_addr, app_state)
        .await
        .expect("[boilermaker_web::main] Failed to start app!");

    app.run()
        .await
        .expect("[boilermaker_web::main] Failed to run app!")
}
