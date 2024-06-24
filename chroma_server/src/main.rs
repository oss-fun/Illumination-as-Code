use chroma_server::parse_count_to_chroma;

use actix_rt::time::sleep;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone)]
struct AppState {
    data: Arc<Mutex<String>>,
    receiving: Arc<Mutex<bool>>,
}

#[derive(Serialize, Deserialize)]
struct Chroma {
    red: u8,
    green: u8,
    blue: u8,
}
#[derive(Serialize, Deserialize)]
struct Count {
    red: usize,
    green: usize,
    blue: usize,
}

// post raspi to this server
async fn start_receiving(state: web::Data<AppState>, body: String) -> impl Responder {
    match body.trim().parse::<u64>() {
        Ok(time) => {
            // 内部状態のクリア
            {
                let mut data = state.data.lock().unwrap();
                data.clear();
                let mut receiving = state.receiving.lock().unwrap();
                *receiving = true; // 受信状態をオンにする
            }

            let _data = state.data.clone();
            let receiving = state.receiving.clone();
            let receive_task = actix_rt::spawn(async move {
                let start_time = std::time::Instant::now();

                while start_time.elapsed() < Duration::new(time, 0) {
                    // 1秒ごとにデータ受信をチェック
                    sleep(Duration::from_nanos(100)).await;
                }

                // 10秒経過後に受信状態をオフにす
                let mut receiving = receiving.lock().unwrap();
                *receiving = false;
            });

            receive_task.await.unwrap();

            HttpResponse::Ok().body(format!(
                "Receiving data for {} seconds is done",
                time.to_string()
            ))
        }
        Err(e) => {
            println!("Error: {}", e);
            return HttpResponse::BadRequest().body("Invalid time: must be a number");
        }
    }
}

// get raspi from this server
async fn get_chroma(state: web::Data<AppState>, body: String) -> impl Responder {
    let data = state.data.lock().unwrap();

    println!("{:?}", *data); // log

    match body.trim().parse::<usize>() {
        Ok(time) => {
            let red = parse_count_to_chroma(&data, "R", time);
            let green = parse_count_to_chroma(&data, "G", time);
            let blue = parse_count_to_chroma(&data, "B", time);

            HttpResponse::Ok().json(Count {
                red: red,
                green: green,
                blue: blue,
            })
        }
        Err(e) => {
            println!("Error: {}", e);
            HttpResponse::BadRequest().body("Invalid time: must be a number")
        }
    }
}

async fn get_chroma_static(_state: web::Data<AppState>, _body: String) -> impl Responder {
    HttpResponse::Ok().json(Chroma {
        red: 255,
        green: 85,
        blue: 0,
    })
}

// post vm to this server
async fn receive_data(state: web::Data<AppState>, body: String) -> impl Responder {
    let receiving = state.receiving.lock().unwrap();
    if *receiving {
        let mut data = state.data.lock().unwrap();
        data.push_str(&body);
        HttpResponse::Ok().body("Data received")
    } else {
        HttpResponse::Ok().body("Data ignored: Receiving is off")
    }
}

// test endpoint
async fn vm_up(_state: web::Data<AppState>, body: String) -> impl Responder {
    if body.trim().parse().unwrap() {
        HttpResponse::Ok().body("Success!")
    } else {
        HttpResponse::BadRequest().body("Failed!")
    }
}

async fn root()-> impl Responder{
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = AppState {
        data: Arc::new(Mutex::new(String::new())),
        receiving: Arc::new(Mutex::new(false)), // 初期状態は受信しない
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .route("/start", web::post().to(start_receiving))
            .route("/receive", web::post().to(receive_data))
            .route("/get-chroma", web::post().to(get_chroma))
            .route("/get-chroma-static", web::post().to(get_chroma_static))
            .route("/vm_up", web::post().to(vm_up))
            .route("/",web::post().to(root))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
