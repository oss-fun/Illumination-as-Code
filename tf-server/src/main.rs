use actix_rt::time::sleep;
use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone)]
struct AppState {
    data: Arc<Mutex<String>>,
    receiving: Arc<Mutex<bool>>,
}

#[derive(Serialize, Deserialize)]
pub struct Count {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

async fn vm_up(state: web::Data<AppState>, body: web::Json<Count>) -> impl Responder {
    println!("vm up start..");
    let count = body.into_inner();

    let cpu_values = format!("[\"{}\", \"{}\", \"{}\"]", count.red, count.green, count.blue);

    // コマンドを非同期で実行
    let output = actix_rt::task::spawn_blocking(move || {
        Command::new("sh")
            .current_dir("/home/ubuntu/illumination-as-code/tf-kvm")
            .arg("-c")
            .arg(format!("cd ~/illumination-as-code/tf-kvm && terraform apply -auto-approve -var 'cpu={}'", cpu_values))
            .output()
            .expect("failed to execute process")
    })
    .await
    .expect("task failed");

    // コマンドの実行が完了するまで待機
    if output.status.success() {
        println!("Command executed successfully");
    } else {
        println!("Command failed to execute");
    }

    // 10秒待機
    sleep(Duration::from_secs(10)).await;

    // レスポンスを返す
    format!("Command executed. Waiting period over.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = AppState {
        data: Arc::new(Mutex::new(String::new())),
        receiving: Arc::new(Mutex::new(false)),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/vm-up", web::post().to(vm_up))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
