mod config;
mod db;

use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tracing::{error, info}; // ← Убираем Level
use tracing_subscriber::EnvFilter; // ← Теперь должно работать

use crate::config::Config;

#[tokio::main]
async fn main() {
    // Загружаем конфигурацию
    let config = match Config::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("❌ Ошибка загрузки конфигурации: {}", e);
            std::process::exit(1);
        }
    };

    // Инициализируем логирование с уровнем из конфигурации
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.log_level));

    if let Err(e) = tracing_subscriber::fmt()
        .with_env_filter(filter) // ← Теперь метод доступен
        .try_init()
    {
        eprintln!("Failed to initialize logger: {}", e);
        std::process::exit(1);
    }

    // Подключаемся к базе данных
    let _pool = match db::create_pool(&config).await {
        Ok(pool) => pool,
        Err(e) => {
            error!("❌ Не удалось подключиться к базе данных: {}", e);
            return;
        }
    };

    // Создаем маршрутизатор
    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(health_check));

    // Получаем адрес сервера из конфигурации
    let server_address = config.server_address();

    // Создаем TCP listener
    let listener = match TcpListener::bind(&server_address).await {
        Ok(listener) => {
            info!("🚀 Сервер запускается на http://{}", server_address);
            listener
        }
        Err(e) => {
            error!(
                "❌ Не удалось запустить сервер на {}: {}",
                server_address, e
            );
            return;
        }
    };

    info!("✅ База данных подключена и сервер готов принимать запросы!");
    info!("📊 Конфигурация:");
    info!("  • Хост: {}", config.server_host);
    info!("  • Порт: {}", config.server_port);
    info!("  • Уровень логов: {}", config.log_level);
    info!(
        "  • Макс. соединений к БД: {}",
        config.max_database_connections
    );

    // Запускаем сервер
    if let Err(e) = axum::serve(listener, app).await {
        error!("❌ Ошибка сервера: {}", e);
    }
}

// Главный обработчик
async fn handler() -> &'static str {
    info!("📨 Получен запрос на /");
    "Hello, Corn! Database connection is ready!"
}

// Простейшая проверка здоровья сервера
async fn health_check() -> &'static str {
    info!("🩺 Health check запрос");
    "OK"
}
