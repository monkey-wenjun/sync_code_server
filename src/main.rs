mod config;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use redis::Client;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use log::{info, error, warn};
use crate::config::Settings;

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
    info!("收到POST请求，消息内容: {}", message.message);
    
    let client = data.redis_client.lock().unwrap();
    let mut conn = match client.get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            error!("Redis连接失败: {}", e);
            return HttpResponse::InternalServerError().json(Message {
                message: "服务器内部错误".to_string(),
            });
        }
    };
    
    // 将消息存储到 Redis
    match redis::cmd("SET")
        .arg("last_message")
        .arg(&message.message)
        .query::<()>(&mut conn)
    {
        Ok(_) => {
            info!("消息成功保存到Redis");
            HttpResponse::Ok().json(Message {
                message: "消息已成功保存".to_string(),
            })
        }
        Err(e) => {
            error!("Redis写入失败: {}", e);
            HttpResponse::InternalServerError().json(Message {
                message: "保存失败".to_string(),
            })
        }
    }
}

// GET 处理函数
async fn get_message(data: web::Data<AppState>) -> impl Responder {
    info!("收到GET请求");
    
    let client = data.redis_client.lock().unwrap();
    let mut conn = match client.get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            error!("Redis连接失败: {}", e);
            return HttpResponse::InternalServerError().json(Message {
                message: "服务器内部错误".to_string(),
            });
        }
    };
    
    // 从 Redis 获取消息
    match redis::cmd("GET")
        .arg("last_message")
        .query::<String>(&mut conn)
    {
        Ok(message) => {
            info!("成功从Redis读取消息");
            HttpResponse::Ok().json(Message { message })
        }
        Err(e) => {
            warn!("从Redis读取消息失败: {}", e);
            HttpResponse::Ok().json(Message {
                message: "没有找到消息".to_string(),
            })
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    info!("正在加载配置...");
    
    // 加载配置
    let settings = match Settings::new() {
        Ok(settings) => settings,
        Err(e) => {
            error!("配置加载失败: {}", e);
            return Ok(());
        }
    };
    
    info!("正在启动应用...");
    
    // 创建 Redis 客户端
    let client = match Client::open(settings.redis.url) {
        Ok(client) => {
            info!("Redis客户端创建成功");
            client
        }
        Err(e) => {
            error!("Redis客户端创建失败: {}", e);
            return Ok(());
        }
    };
    
    // 创建应用状态
    let app_state = web::Data::new(AppState {
        redis_client: Mutex::new(client),
    });

    let bind_address = format!("{}:{}", settings.server.host, settings.server.port);
    info!("开始监听 {}", bind_address);
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/api/message", web::post().to(post_message))
            .route("/api/message", web::get().to(get_message))
    })
    .bind(&bind_address)?
    .run()
    .await
}
