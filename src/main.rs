mod server;

use server::ChatServer;

use actix::prelude::*;

// use std::net::TcpListener;
use futures_util::stream::once;
use futures_util::StreamExt;
use std::net;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;

#[derive(Message)]
#[rtype(result = "()")]
struct TcpConnect(pub WebSocketStream<TcpStream>, pub net::SocketAddr);

struct MyActor;

// Provide Actor implementation for our actor
impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

/// Define handler for `Ping` message
impl Handler<TcpConnect> for MyActor {
    type Result = ();

    fn handle(&mut self, msg: TcpConnect, ctx: &mut Context<Self>) -> Self::Result {
        Box::new(async move {
            // Some async computation
            println!("--");
            let (write, read) = msg.0.split();
            read.forward(write)
                .await
                .expect("Failed to forward message")
            // ()
        });
    }
}

#[actix::main]
async fn main() {
    // let system = actix::System::with_tokio_rt(|| tokio::runtime::Runtime::new().unwrap());
    // system.block_on(async {
    let actor_addr = MyActor.start();
    let listener = TcpListener::bind("127.0.0.1:2345").await.unwrap();
    while let Ok((stream, addr)) = listener.accept().await {
        // match listener.accept().await {
        // Ok((socket, addr)) => {
        let ws_stream = tokio_tungstenite::accept_async(stream).await;
        match ws_stream {
            Ok(ws) => {
                println!("ok");
                actor_addr.do_send(TcpConnect(ws, addr));
            }
            Err(e) => {
                println!("err");
                // Messages::Pong
            }
        };
    }
    //     MyActor::create(|ctx: &mut Context<MyActor>| {
    //         let actor_addr = ctx.address();

    //         ctx.add_message_stream(once(async move {

    //             Messages::Pong

    //             // let t = listener
    //             //     .accept()
    //             //     .await
    //             //     // .map_err(|_| ())
    //             //     .map(|(stream, _addr)| async { tokio_tungstenite::accept_async(stream).await })
    //             //     .map(|tt| async {
    //             //         let c = tt.await;
    //             //         c
    //             //     });
    //             // // .map_err(|_| ());

    //             // if let Ok(s) = t {
    //             //     let ws = s.await;
    //             //     match ws {
    //             //         Ok(ws) => Messages::Ping,
    //             //         Err(e) => Messages::Pong,
    //             //     }
    //             // } else {
    //             //     Messages::Pong
    //             // }
    //         }));
    //         MyActor
    //     });
    // });
    // system.run().unwrap();
}
