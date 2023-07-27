#![allow(clippy::unused_async)]

use std::{
    fmt::Debug,
    fs::{read, read_to_string},
    net::{Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
    task::Poll,
};

use anyhow::{Context, Result};
use axum::{
    body::{boxed, Body},
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    http::{header, Request, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use futures::future::BoxFuture;
use tokio::sync::broadcast::Sender;
use tower::{Layer, Service};
use tower_http::trace::TraceLayer;

use crate::Event;

pub async fn create(root: &Path, port: u16, tx: Sender<Event>) -> Result<()> {
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, port));
    let root = root.canonicalize()?;
    axum::Server::bind(&addr)
        .serve(
            router(&root, port, tx)
                .layer(TraceLayer::new_for_http())
                .layer(StaticDirLayer { root })
                .into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .context("Failed to start server")
}

fn router(_root: &Path, port: u16, tx: Sender<Event>) -> Router {
    Router::new()
        .route("/dev_server/ws", get(ws_handler))
        .with_state(tx)
        .route("/dev_server/livereload.js", get(livereload_handler))
        .with_state(port)
}

#[derive(Clone, Default)]
pub struct StaticDirLayer {
    root: PathBuf,
}

impl<S> Layer<S> for StaticDirLayer {
    type Service = StaticDir<S>;

    fn layer(&self, inner: S) -> Self::Service {
        StaticDir {
            root: self.root.clone(),
            inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StaticDir<S> {
    inner: S,
    root: PathBuf,
}

impl<S> Service<Request<Body>> for StaticDir<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = S::Response;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::result::Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let path = self.root.join(&req.uri().path()[1..]);
        let (path, is_html) = if path.exists() && path.is_dir() {
            (path.join("index.html"), true)
        } else {
            (
                path.clone(),
                path.extension().map(|e| e == "html").unwrap_or(false),
            )
        };

        let file = if is_html {
            match read_to_string(&path) {
                Ok(mut file) => {
                    file.push_str(r#"<script src="/dev_server/livereload.js"></script>"#);
                    Ok(file.into_bytes())
                }
                Err(err) => Err(err),
            }
        } else {
            read(&path)
        };

        let file = match file {
            Ok(file) => file,
            Err(_) => {
                let future = self.inner.call(req);
                return Box::pin(async move {
                    let response: Response = future.await?;
                    Ok(response)
                });
            }
        };

        let res = Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(file))
            .unwrap();

        Box::pin(async move { Ok(res.map(boxed)) })
    }
}

async fn livereload_handler(State(port): State<u16>) -> impl IntoResponse {
    let str = include_str!("livereload.js");
    let str = str.replace("{{PORT}}", &port.to_string());

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/javascript")],
        str,
    )
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(tx): State<Sender<Event>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tx, addr))
}

async fn handle_socket(mut socket: WebSocket, tx: Sender<Event>, addr: SocketAddr) {
    tracing::debug!("{addr} connected");
    let mut rx = tx.subscribe();

    while let Ok(event) = rx.recv().await {
        if let Err(e) = match event {
            Event::Reload => socket.send(Message::Text("reload".to_string())).await,
            Event::Shutdown => socket.send(Message::Text("shutdown".to_string())).await,
        } {
            tracing::info!("Failed to send message to {addr}: {e}");
            break;
        }
    }
}
