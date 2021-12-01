use actix::{Actor, Context, Handler, Message};

#[derive(Message)]
#[rtype(result = "usize")]
struct Sum(usize, usize);

struct Calculator;

impl Actor for Calculator {
    type Context = Context<Self>;
}

impl Handler<Sum> for Calculator {
    type Result = usize;

    fn handle(&mut self, msg: Sum, _ctx: &mut Context<Self>) -> Self::Result {
        msg.0 + msg.1
    }
}

#[actix::main] // <- starts the actor system and blocks until future resolves
async fn main() {
    let addr = Calculator.start();

    if let Ok(res) = addr.send(Sum(3, 4)).await {
        println!("Sum: {}", res);
    } else {
        eprintln!("Communication to the actor has failed");
    }
}
