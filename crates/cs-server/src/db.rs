use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing::{error, info};

use crate::config::Config;

/// Создает и возвращает пул соединений с базой данных
pub async fn create_pool(config: &Config) -> Result<PgPool, Box<dyn std::error::Error>> {
    let database_url = &config.database_url;
    let max_connections = config.max_database_connections;

    // Маскируем пароль в URL для логов
    let masked_url = mask_password(database_url);
    info!("Подключение к базе данных: {}", masked_url);
    info!("Максимальное количество соединений: {}", max_connections);

    info!("Создание пула соединений...");
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await
        .map_err(|e| {
            error!("Ошибка подключения к БД: {}", e);
            error!("Проверьте:");
            error!("1. Запущен ли Docker контейнер (docker-compose up -d)");
            error!("2. Правильный ли DATABASE_URL в .env файле");
            error!("Текущий DATABASE_URL: {}", masked_url);
            e
        })?;

    info!("Проверка соединения...");
    sqlx::query("SELECT 1").execute(&pool).await.map_err(|e| {
        error!("БД недоступна после подключения: {}", e);
        e
    })?;

    info!("✅ Успешное подключение к базе данных");
    Ok(pool)
}

// Функция для маскировки пароля в URL
fn mask_password(url: &str) -> String {
    if let Some(at_pos) = url.find('@') {
        if let Some(colon_pos) = url[..at_pos].rfind(':') {
            let mut masked = url.to_string();
            masked.replace_range(colon_pos + 1..at_pos, "***");
            return masked;
        }
    }
    url.to_string()
}
