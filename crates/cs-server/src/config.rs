use std::env;

use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub max_database_connections: u32,
    pub log_level: String,
    pub postgres_host_port: u16, // Добавляем порт хоста
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // Для отладки: покажем все переменные окружения
        println!("🔍 Проверка переменных окружения:");

        // Загружаем переменные из .env файла
        match dotenvy::dotenv() {
            Ok(_) => println!("✅ .env файл загружен успешно"),
            Err(e) => println!("⚠️  Не удалось загрузить .env файл: {}", e),
        }

        // Отладочный вывод всех переменных
        for (key, value) in env::vars() {
            if key.contains("POSTGRES") || key.contains("DATABASE") {
                if key.contains("PASSWORD") {
                    println!("   {} = ********", key);
                } else {
                    println!("   {} = {}", key, value);
                }
            }
        }

        // Читаем отдельные переменные для БД
        let postgres_user = env::var("POSTGRES_USER").unwrap_or_else(|_| {
            println!("⚠️  POSTGRES_USER не найден, использую значение по умолчанию: corn_user");
            "corn_user".to_string()
        });

        let postgres_password = env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| {
            println!(
                "⚠️  POSTGRES_PASSWORD не найден, использую значение по умолчанию: corn_password"
            );
            "corn_password".to_string()
        });

        let postgres_db = env::var("POSTGRES_DB").unwrap_or_else(|_| {
            println!("⚠️  POSTGRES_DB не найден, использую значение по умолчанию: corn_db");
            "corn_db".to_string()
        });

        // Порты
        let postgres_port = env::var("POSTGRES_PORT").unwrap_or_else(|_| {
            println!("⚠️  POSTGRES_PORT не найден, использую значение по умолчанию: 5432");
            "5432".to_string()
        });

        let postgres_host_port = env::var("POSTGRES_HOST_PORT").unwrap_or_else(|_| {
            println!("⚠️  POSTGRES_HOST_PORT не найден, использую значение по умолчанию: 8102");
            "8102".to_string()
        });

        // Формируем DATABASE_URL - используем порт хоста для подключения
        let database_url = format!(
            "postgres://{}:{}@localhost:{}/{}",
            postgres_user, postgres_password, postgres_host_port, postgres_db
        );

        println!(
            "🔗 Сформированный DATABASE_URL: {}",
            mask_password(&database_url)
        );
        println!("📡 Подключение через порт хоста: {}", postgres_host_port);

        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| {
            println!("⚠️  SERVER_HOST не найден, использую значение по умолчанию: 127.0.0.1");
            "127.0.0.1".to_string()
        });

        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| {
                println!("⚠️  SERVER_PORT не найден, использую значение по умолчанию: 3000");
                "3000".to_string()
            })
            .parse::<u16>()
            .map_err(|e| format!("Неверный формат SERVER_PORT: {}", e))?;

        let max_database_connections = env::var("MAX_DATABASE_CONNECTIONS")
            .unwrap_or_else(|_| {
                println!(
                    "⚠️  MAX_DATABASE_CONNECTIONS не найден, использую значение по умолчанию: 10"
                );
                "10".to_string()
            })
            .parse::<u32>()
            .map_err(|e| format!("Неверный формат MAX_DATABASE_CONNECTIONS: {}", e))?;

        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| {
            println!("⚠️  LOG_LEVEL не найден, использую значение по умолчанию: info");
            "info".to_string()
        });

        let config = Config {
            database_url,
            server_host,
            server_port,
            max_database_connections,
            log_level,
            postgres_host_port: postgres_host_port
                .parse::<u16>()
                .map_err(|e| format!("Неверный формат POSTGRES_HOST_PORT: {}", e))?,
        };

        info!("Загружена конфигурация:");
        info!("  • Хост: {}", config.server_host);
        info!("  • Порт: {}", config.server_port);
        info!(
            "  • Макс. соединений к БД: {}",
            config.max_database_connections
        );
        info!("  • Уровень логов: {}", config.log_level);
        info!(
            "  • Порт PostgreSQL на хосте: {}",
            config.postgres_host_port
        );
        info!("  • БД: {}", mask_password(&config.database_url));

        Ok(config)
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
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
