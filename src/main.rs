use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

struct AppState {
    redis: Arc<redis::Client>,
}

#[derive(Deserialize)]
struct HelloQuery {
    name: String,
}

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

async fn hello(
    query: web::Query<HelloQuery>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut conn = match data.redis.get_multiplexed_async_connection().await {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError().json(HelloResponse {
                message: "Redis connection failed".to_string(),
            })
        }
    };

    let _: Result<(), _> = conn
        .incr(format!("greetings:{}", query.name), 1)
        .await;

    HttpResponse::Ok().json(HelloResponse {
        message: format!("Hello, {}!", query.name),
    })
}

async fn health(data: web::Data<AppState>) -> impl Responder {
    match data.redis.get_multiplexed_async_connection().await {
        Ok(mut conn) => {
            match redis::cmd("PING").query_async::<_, String>(&mut conn).await {
                Ok(_) => HttpResponse::Ok().json(HealthResponse {
                    status: "healthy".to_string(),
                }),
                Err(_) => HttpResponse::ServiceUnavailable().json(HealthResponse {
                    status: "unhealthy - redis not responding".to_string(),
                }),
            }
        }
        Err(_) => HttpResponse::ServiceUnavailable().json(HealthResponse {
            status: "unhealthy - cannot connect to redis".to_string(),
        }),
    }
}

async fn readiness(data: web::Data<AppState>) -> impl Responder {
    match data.redis.get_multiplexed_async_connection().await {
        Ok(_) => HttpResponse::Ok().json(HealthResponse {
            status: "ready".to_string(),
        }),
        Err(_) => HttpResponse::ServiceUnavailable().json(HealthResponse {
            status: "not ready".to_string(),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".to_string());
    
    let redis_client = redis::Client::open(redis_url)
        .expect("Failed to create Redis client");

    let app_state = web::Data::new(AppState {
        redis: Arc::new(redis_client),
    });

    println!("Server running on http://0.0.0.0:3000");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/hello", web::get().to(hello))
            .route("/health", web::get().to(health))
            .route("/readiness", web::get().to(readiness))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}