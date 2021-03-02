mod server;

use server::ChatServer;

use actix::prelude::*;

// use std::net::TcpListener;
use futures_util::stream::once;
use std::net;
use tokio::net::TcpListener;
use tungstenite::server::accept;

#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
struct Ping;

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
impl Handler<Ping> for MyActor {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> Self::Result {
        println!("Ping received");

        Ok(true)
    }
}

#[actix::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:2345").await.unwrap();

    MyActor::create(|ctx: &mut Context<MyActor>| {
        // ctx.add_message_stream(listener.accept
        // while let Ok((stream, addr)) = listener.accept().await {
        //     tokio::spawn(handle_connection(state.clone(), stream, addr));
        // }
        // ()
        // });
        // ctx.add_message_stream(fut)
        ctx.add_message_stream(once(async move {
            // listener.accept().await;
            match listener.accept().await {
                Ok((_socket, addr)) => println!("new client: {:?}", addr),
                Err(e) => println!("couldn't get client: {:?}", e),
            }
            Ping
        }));
        // ctx.add_message_stream(listener.incoming().map_err(|_| ()).map(|st| {
        //     let addr = st.peer_addr().unwrap();
        //     // TcpConnect(st, addr)
        // }));
        MyActor
    });
}
