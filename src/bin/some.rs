use actix::prelude::*;

// use std::net::TcpListener;
use futures_util::stream::once;
use std::net;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;

#[derive(Message, Debug)]
#[rtype(result = "()")]
enum Messages {
    Ping,
    Pong,
}

#[derive(Message)]
#[rtype(result = "()")]
struct TcpConnect(pub WebSocketStream<TcpStream>, pub net::SocketAddr);

struct Server;

// Provide Actor implementation for our actor
impl Actor for Server {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

/// Define handler for `Ping` message
impl Handler<Messages> for Server {
    type Result = ();

    fn handle(&mut self, msg: Messages, ctx: &mut Context<Self>) -> Self::Result {
        println!("Ping received {:?}", msg);
    }
}

#[actix::main]
async fn main() {
    let actor_addr = Server.start();
    let listener = TcpListener::bind("127.0.0.1:2345").await.unwrap();
    while let Ok((stream, addr)) = listener.accept().await {
        let ws_stream = tokio_tungstenite::accept_async(stream).await;
        match ws_stream {
            Ok(ws) => {
                println!("ok");
                actor_addr.do_send(Messages::Ping)
            }
            Err(e) => {
                println!("err");
            }
        };
    }
}
