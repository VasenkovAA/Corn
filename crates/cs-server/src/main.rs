use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tracing::{Level, error, info};

#[tokio::main]
async fn main() {
    // Инициализируем логирование
    if let Err(e) = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .try_init()
    {
        eprintln!("Failed to initialize logger: {}", e);
        std::process::exit(1);
    }

    // Создаем маршрутизатор
    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(health_check));

    // Создаем TCP listener
    let listener = match TcpListener::bind("127.0.0.1:3000").await {
        Ok(listener) => {
            info!("🚀 Сервер запускается на http://127.0.0.1:3000");
            listener
        }
        Err(e) => {
            error!("❌ Не удалось запустить сервер: {}", e);
            return;
        }
    };

    info!("📡 Подключение к БД будет настроено завтра");
    info!("✅ Сервер готов принимать запросы!");

    // Запускаем сервер
    if let Err(e) = axum::serve(listener, app).await {
        error!("❌ Ошибка сервера: {}", e);
    }
}

// Главный обработчик
async fn handler() -> &'static str {
    info!("📨 Получен запрос на /");
    "Hello, ConnectSphere!\nDatabase connection will be ready tomorrow!"
}

// Простейшая проверка здоровья сервера
async fn health_check() -> &'static str {
    info!("🩺 Health check запрос");
    "OK"
}
