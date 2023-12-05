use futures_util::sink::SinkExt;
use poem::{
    get, handler,
    listener::TcpListener,
    web::{
        websocket::{Message, WebSocket},
        Path,
    },
    IntoResponse, Route, Server,
};

#[handler]
fn ws(Path(first): Path<String>, Path(second): Path<String>, ws: WebSocket) -> impl IntoResponse {
    ws.on_upgrade(move |mut socket| async move {
        println!("first: {}, second: {}", first, second);
        socket.send(Message::text("Done")).await.ok();
    })
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }

    let app = Route::new().at("/ws/:first/:second", get(ws));

    println!("Starting server");

    Server::new(TcpListener::bind("0.0.0.0:3123"))
        .run(app)
        .await
}
