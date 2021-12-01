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

fn main() -> std::io::Result<()> {
    let rt = Runtime::new()?;
    rt.block_on(async {
        let future = task::spawn(print1());
        print2().await;
        future.await.unwrap();
    });
    Ok(())
}

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
