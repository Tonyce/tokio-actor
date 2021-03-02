use actix::prelude::*;
struct MyActor {
    count: usize,
}
impl Actor for MyActor {
    type Context = Context<Self>;
}

struct Ping(usize);

impl Message for Ping {
    type Result = usize;
}
impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}

#[actix::main]
async fn main() {
    // let system = actix::System::with_tokio_rt(|| {
    //     let rt = tokio::runtime::Runtime::new().unwrap();
    //     rt
    // });

    // system.block_on(async {
    let addr = MyActor { count: 10 }.start();

    // send message and get future for result
    let _res = addr.send(Ping(10)).await;
    let res = addr.send(Ping(10)).await;

    // handle() returns tokio handle
    println!("RESULT: {}", res.unwrap());
    // System::current().stop();
    // })

    Actor::create(|ctx: &mut Context<MyActor>| MyActor { count: 10 });
}
