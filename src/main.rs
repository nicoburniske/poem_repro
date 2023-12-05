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
use url::Url;

#[handler]
fn ws(Path(first): Path<String>, Path(second): Path<String>, ws: WebSocket) -> impl IntoResponse {
    ws.on_upgrade(move |mut socket| async move {
        println!("Path Parameters | first: {}, second: {}", first, second);
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

    let address = "0.0.0.0:3123";

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        let url = format!("ws://{address}/ws/first/second");
        println!("Connecting to {}", url);

        let url = Url::parse(&url).expect("Valid URL");

        let maybe_error = tokio_tungstenite::connect_async(url)
            .await
            .map_err(|e| match e {
                tokio_tungstenite::tungstenite::Error::Http(response) => {
                    let body = response
                        .body()
                        .clone()
                        .and_then(|b| String::from_utf8(b).ok());

                    format!(
                        "Expected Connect Http Error: Status {:?}, Body {:?}",
                        response.status(),
                        body
                    )
                }
                e => format!("Unknown Error: ${}", e.to_string()),
            })
            .err();

        if let Some(error) = maybe_error {
            println!("{}", error);
        } else {
            println!("Connected. This shouldn't happen.");
        }
    });

    Server::new(TcpListener::bind(address)).run(app).await
}
