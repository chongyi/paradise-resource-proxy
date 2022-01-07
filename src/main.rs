#[macro_use]
extern crate serde;

use std::env;

use axum::Router;
use tokio::runtime::Runtime;
pub use error::{Error, Result};

mod target;
pub mod error;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    #[cfg(target_family = "windows")]
    start_server_with_runtime();

    #[cfg(target_family = "unix")]
    {
        use daemonize_me::Daemon;

        let is_daemon = env::var("PARADISE_PROXY_DAEMON")
            .ok()
            .map(|res| res == "true")
            .unwrap_or(false);

        if is_daemon {
            let daemon = Daemon::new().work_dir(".").start();

            match daemon {
                Ok(_) => start_server_with_runtime(),
                Err(err) => eprintln!("Err: {}", err),
            }
        } else {
            start_server_with_runtime()
        }
    }
}

fn start_server_with_runtime() {
    log::info!("Create server runtime.");
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        start_server().await;
    })
}

async fn start_server() {
    log::info!("start server now.");

    let router = Router::new()
        .nest("/pornlulu", target::pornlulu::routes())
        .nest("/baihua", target::baihua::routes());

    axum::Server::bind(&"0.0.0.0:8010".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
