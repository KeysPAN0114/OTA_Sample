use actix_web::{web,App,HttpServer,HttpResponse,Responder};
use serde::{Deserialize,Serialize};
use chrono::Local;
use local_ip_address::local_ip;
use semver::Version;

#[derive(Debug,Serialize,Deserialize)]
struct PostData {
    // timetamp: String,
    firmware: String,
    version: String,
    model:String,
    // message: String,
}

#[derive(Debug,Serialize,Deserialize)]
struct ResponseData {
    timetamp: String,
    firmware: String,
    model:String,
    version: String,
    url: String,
    message: String,
}

fn compare(version1: &str, version2: &str) -> i32 { 
        let com_version1 = Version::parse(version1).expect("Invalid version");
        let com_version2 = Version::parse(version2).expect("Invalid version");
        com_version1.cmp(&com_version2)
    }

async fn handle_post(data: web::Json<PostData>) -> impl Responder {
    println!{"Client post data:{:?}",data};
    let respons_data = ResponseData {
        timetamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        firmware: "test_firmware".to_string(),
        model: "ESP32-WROOM".to_string(),
        version: "1.0.1".to_string(),
        url:"http://url/file/20250512_225046_Client.bin".to_string(),
        message: "new versiom".to_string(),
        // firmware: data.firmware.clone(),
        // model: data.model.clone(),
        // version: data.version.clone(),
        // message: data.message.clone(),
    };
    HttpResponse::Ok().json(respons_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let local_ip = local_ip();
    println!("Server start:127.0.0.1:8081");
    println!("Server start:{:?}:8081",local_ip);
    HttpServer::new(|| {
        App::new()
            .route("/api/data", web::post().to(handle_post))
    })
    // .bind("127.0.0.1:8081")?
    .bind("0.0.0.0:8081")?  // 监听所有网卡
    .run()
    .await
}