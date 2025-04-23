use actix_web::{web,App,HttpServer,HttpResponse,Responder};
use serde::{Deserialize,Serialize};
use chrono::Local;

#[derive(Debug,Serialize,Deserialize)]
struct PostData {
    timetamp: String,
    firmware: String,
    version: String,
    model:String,
    message: String,
}

#[derive(Debug,Serialize,Deserialize)]
struct ResponseData {
    timetamp: String,
    firmware: String,
    model:String,
    version: String,
    message: String,
}

async fn handle_post(data: web::Json<PostData>) -> impl Responder {
    println!{"Client post data:{:?}",data};
    let respons_data = ResponseData {
        timetamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        firmware: data.firmware.clone(),
        model: data.model.clone(),
        version: data.version.clone(),
        message: data.message.clone(),
    };
    HttpResponse::Ok().json(respons_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server start:127.0.0.1:8081");
    HttpServer::new(|| {
        App::new()
            .route("/api/data", web::post().to(handle_post))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}