use actix::{Actor, Context, System};

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("I am alive!");
        System::current().stop();
    }
}

fn main() -> std::io::Result<()> {
    let system = System::new();
    system.block_on(async { MyActor.start() });
    system.run()
}

/*
use tokio::{
    runtime::Runtime,
    task,
    time::{sleep, Duration},
};

async fn print1() {
    sleep(Duration::from_secs(2)).await;
    println!("[1]");
}

async fn print2() {
    println!("[2]");
}
*/

/*
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
*/
