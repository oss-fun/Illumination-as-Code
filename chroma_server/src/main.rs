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

// post raspi to this server
async fn start_receiving(state: web::Data<AppState>) -> impl Responder {
    // 内部状態のクリア
    {
        let mut data = state.data.lock().unwrap();
        data.clear();
        let mut receiving = state.receiving.lock().unwrap();
        *receiving = true; // 受信状態をオンにする
    }

    let data = state.data.clone();
    let receiving = state.receiving.clone();
    actix_rt::spawn(async move {
        let start_time = std::time::Instant::now();

        while start_time.elapsed() < Duration::new(15, 0) {
            // 1秒ごとにデータ受信をチェック
            sleep(Duration::from_nanos(100)).await;
        }

        // 10秒経過後に受信状態をオフにする
        let mut receiving = receiving.lock().unwrap();
        *receiving = false;
    });
    HttpResponse::Ok().body("Started receiving data for 10 seconds")
}

// get raspi from this server
async fn get_chroma(state: web::Data<AppState>) -> impl Responder {
    let data = state.data.lock().unwrap();
    println!("{:?}", *data);
    // 1: 4,2: 8,4:12
    let r_count: usize = data.match_indices("R").count();
    let g_count: usize = data.match_indices("G").count();
    let b_count: usize = data.match_indices("B").count();
    println!("r:{} g:{} b:{}", r_count, g_count, b_count);
    let (red, green, blue) = parse_count_to_chroma(r_count, g_count, b_count);
    HttpResponse::Ok().json(Chroma {
        red: red,
        green: green,
        blue: blue,
    })
}

fn parse_count_to_chroma(r_count: usize, g_count: usize, b_count: usize) -> (u8, u8, u8) {
    let red: u8 = if (r_count / 5 * 85) < 255 {
        (r_count / 5 * 85) as u8
    } else {
        255
    };
    let green: u8 = if (g_count / 5 * 85) < 255 {
        (g_count / 5 * 85) as u8
    } else {
        255
    };
    let blue: u8 = if (b_count / 5 * 85) < 255 {
        (b_count / 5 * 85) as u8
    } else {
        255
    };
    (red, green, blue)
}

// post vm to this server
async fn receive_data(state: web::Data<AppState>, body: String) -> impl Responder {
    let receiving = state.receiving.lock().unwrap();
    if *receiving {
        let mut data = state.data.lock().unwrap();
        data.push_str(&body);
        println!("{:?}", *data);
        HttpResponse::Ok().body("Data received")
    } else {
        HttpResponse::Ok().body("Data ignored: Receiving is off")
    }
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
            .route("/start", web::get().to(start_receiving))
            .route("/receive", web::post().to(receive_data))
            .route("/get-chroma", web::get().to(get_chroma))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
