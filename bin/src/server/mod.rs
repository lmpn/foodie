use std::net::SocketAddr;
use std::time::Duration;

use axum::{
    http::{
        header::{ACCEPT, ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, ORIGIN},
        Method,
    },
    routing::get,
    Router,
};
use axum_server::Handle;
use tower_http::cors::{Any, CorsLayer};
use tracing::{event, Level};

use crate::state::State;
use crate::{configuration::Configuration, web};
pub struct Server {
    handle: Option<Handle>,
    address: SocketAddr,
    router: Router<()>,
}

impl Server {
    pub fn new(state: State, configuration: &Configuration) -> Self {
        let router = Self::create_router(state, configuration);
        let sock_address = SocketAddr::from(configuration.address());
        Self {
            handle: Some(Handle::new()),
            address: sock_address,
            router,
        }
    }

    pub fn serve(&mut self) {
        let handle = match &self.handle {
            Some(handle) => handle.clone(),
            None => {
                let handle = Handle::new();
                self.handle = Some(handle.clone());
                handle
            }
        };
        let server = axum_server::bind(self.address)
            .handle(handle)
            .serve(self.router.clone().into_make_service());
        tokio::spawn(async {
            event!(Level::INFO, "Starting server");
            server.await.expect("Error starting server");
        });
    }

    pub async fn reload(&mut self, state: State, configuration: Configuration) {
        let sock_address = SocketAddr::from(configuration.address());
        if self.address == sock_address {
            return;
        }
        self.stop().await;

        self.address = sock_address;
        self.router = Self::create_router(state, &configuration);

        self.serve()
    }

    pub async fn stop(&mut self) {
        if self.handle.is_none() {
            return;
        }
        let handle = self.handle.take().unwrap();
        handle.graceful_shutdown(Some(Duration::from_secs(3)));
        let mut conn_count = handle.connection_count();
        while conn_count > 0 {
            tokio::time::sleep(Duration::from_secs(1)).await;
            conn_count = handle.connection_count();
        }
        event!(Level::INFO, "Stopping server");
    }

    fn create_router(state: State, configuration: &Configuration) -> Router {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET])
            .allow_headers([AUTHORIZATION, ORIGIN, ACCEPT, ACCESS_CONTROL_ALLOW_ORIGIN]);
        Router::new()
            .route("/", get(hello_world))
            .merge(web::recipes::router(state.clone(), configuration))
            .merge(web::authorization::router(state, configuration))
            .layer(cors)
            .fallback(web::handler_404)
    }
}

async fn hello_world() -> &'static str {
    "Hello world!"
}
