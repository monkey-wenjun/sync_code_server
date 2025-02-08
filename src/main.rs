use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use redis::Client;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// 定义消息结构体
#[derive(Serialize, Deserialize)]
struct Message {
    message: String,
}

// Redis 客户端包装器
struct AppState {
    redis_client: Mutex<Client>,
}

// POST 处理函数
async fn post_message(
    data: web::Data<AppState>,
    message: web::Json<Message>,
) -> impl Responder {
    let client = data.redis_client.lock().unwrap();
    let mut conn = client.get_connection().unwrap();
    
    // 将消息存储到 Redis
    let _: () = redis::cmd("SET")
        .arg("last_message")
        .arg(&message.message)
        .query(&mut conn)
        .unwrap();

    HttpResponse::Ok().json(Message {
        message: "消息已成功保存".to_string(),
    })
}

// GET 处理函数
async fn get_message(data: web::Data<AppState>) -> impl Responder {
    let client = data.redis_client.lock().unwrap();
    let mut conn = client.get_connection().unwrap();
    
    // 从 Redis 获取消息
    let message: String = redis::cmd("GET")
        .arg("last_message")
        .query(&mut conn)
        .unwrap_or_else(|_| "没有找到消息".to_string());

    HttpResponse::Ok().json(Message { message })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 创建 Redis 客户端
    let client = Client::open("redis://127.0.0.1:6379").unwrap();
    
    // 创建应用状态
    let app_state = web::Data::new(AppState {
        redis_client: Mutex::new(client),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/api/message", web::post().to(post_message))
            .route("/api/message", web::get().to(get_message))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
