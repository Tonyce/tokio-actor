use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
struct Ping;

pub struct ChatServer;

impl Default for ChatServer {
    fn default() -> Self {
        ChatServer
    }
}

// Provide Actor implementation for our actor
impl Actor for ChatServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

impl Handler<Ping> for ChatServer {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> Self::Result {
        println!("Ping received");

        Ok(true)
    }
}
