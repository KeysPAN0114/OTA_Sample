use actix_web::{web,App,HttpServer,HttpResponse,Responder};
use serde::{Deserialize,Serialize};
use chrono::Local;
use local_ip_address::local_ip;
use semver::{Op, Version};
use std::cmp::Ordering;
use std::ops::Sub;
use actix_multipart::Multipart;
use futures_util::StreamExt;
use std::fs::File;
use std::io::Write;
use std::thread;

const Acces_ip:&str = "192.168.1.1";

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

#[derive(Debug, Serialize, Deserialize)]
struct PostDataHtml {
    message: String,
    timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseDataHtml {
    status: String,
    received_message: String,
    server_time: String,
    request_via: String,
    file_path: Option<String>,
    url: String,
}

/*
    function: 版本号对比
*/
#[warn(dead_code)]
fn compare(version1: &str, version2: &str) -> i32 { 
        let com_version1 = Version::parse(version1).expect("Invalid version");
        let com_version2 = Version::parse(version2).expect("Invalid version");
        match com_version1.cmp(&com_version2) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

async fn handle_post(data: web::Json<PostData>) -> impl Responder {
    println!{"Client post data:{:?}",data};
    let respons_data = ResponseData {
        timetamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        firmware: "test_firmware".to_string(),
        model: "ESP32-WROOM".to_string(),
        version: "1.0.1".to_string(),
        url:"http://127.0.0.1/file/20250512_225046_Client.bin".to_string(),
        message: "new versiom".to_string(),
        // firmware: data.firmware.clone(),
        // model: data.model.clone(),
        // version: data.version.clone(),
        // message: data.message.clone(),
    };
    HttpResponse::Ok().json(respons_data)
}

fn get_upload_dir() -> String {
    if cfg!(target_os = "windows") {
        let dir = "D:\\uploadfiles";
        std::fs::create_dir(dir).unwrap_or_else(|e| {
            eprintln!("Failed to create directory: {}", e);
        });
        dir.to_string()
    } else {
        let dir = "/var/www/esp32_dir/ota_dir/esp32_ota";
        std::fs::create_dir(dir).unwrap_or_else(|e| {
            eprintln!("Failed to create directory: {}", e);
        });
        dir.to_string()
    }
}
async fn upload_file(mut payload: Multipart) -> impl Responder {
    let mut file_path = None;
    let mut url_get:Option<String> = None;

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(e) => {
                eprintln!("Multipart error: {}", e);
                continue;
            }
        };

        let content_disposition = field.content_disposition();
        let filename = match content_disposition.get_filename() {
            Some(name) => name.to_string(),
            None => continue,
        };

        let upload_dir = get_upload_dir();
        let safe_filename =  filename.replace("../", "");
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let save_path = if cfg!(target_os = "windows") {
            format!("{}\\{}_{}", upload_dir, timestamp,safe_filename)
        } else {
            format!("{}/{}_{}", upload_dir, timestamp, safe_filename)
        };

        // 创建文件并写入内容
        let mut file = File::create(&save_path).unwrap_or_else(|e| {
            eprintln!("Failed to create file: {}", e);
            panic!("File creation failed");
        });
        
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file.write_all(&data).unwrap_or_else(|e| {
                eprintln!("Failed to write to file: {}", e);
                panic!("File write failed");
            });
        }

        file_path = Some(save_path);
        url_get = Some(format!("http://{}/file/{}_{}",Acces_ip,timestamp,safe_filename));
    }
    HttpResponse::Ok().json(ResponseDataHtml {
        status: "success".to_string(),
        received_message: "File uploaded successfully".to_string(),
        server_time: Local::now().to_rfc3339(),
        request_via: "Rust backend".to_string(),
        file_path,
        url:url_get.expect("reason").to_string(),
    })
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let local_ip = local_ip();
    println!("Server start:127.0.0.1:8081");
    println!("Server start:{:?}:8081",local_ip);

    HttpServer::new(|| {
        App::new()
            .route("/api/data", web::post().to(handle_post))
            .route("/api/upload",web::post().to(upload_file))
    })
    // .bind("127.0.0.1:8081")?
    .bind("0.0.0.0:8081")?  // 监听所有网卡
    .run()
    .await
}