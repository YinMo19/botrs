# API 集成示例

本示例展示如何在 BotRS 机器人中集成外部 API 服务，包括 HTTP 客户端配置、数据获取、缓存策略以及错误处理。

## 概述

现代机器人通常需要与各种外部服务集成，如天气 API、翻译服务、数据库、第三方平台等。本示例展示如何安全高效地集成这些服务。

## 基础 HTTP 客户端设置

### 创建 HTTP 客户端

```rust
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

pub struct ApiClient {
    http_client: Client,
    base_url: String,
    api_key: Option<String>,
    rate_limiter: RateLimiter,
}

impl ApiClient {
    pub fn new(base_url: String, api_key: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let http_client = ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .user_agent("BotRS/1.0")
            .pool_idle_timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(10)
            .build()?;

        Ok(Self {
            http_client,
            base_url,
            api_key,
            rate_limiter: RateLimiter::new(60, Duration::from_secs(60)), // 每分钟60次请求
        })
    }

    async fn make_request<T>(&self, endpoint: &str) -> Result<T, ApiError>
    where
        T: for<'de> Deserialize<'de>,
    {
        // 等待速率限制
        self.rate_limiter.acquire().await;

        let url = format!("{}/{}", self.base_url, endpoint);
        let mut request = self.http_client.get(&url);

        // 添加 API 密钥
        if let Some(ref api_key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request.send().await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let data: T = response.json().await?;
                Ok(data)
            }
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                // 处理速率限制
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|h| h.to_str().ok())
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(60);

                sleep(Duration::from_secs(retry_after)).await;
                Err(ApiError::RateLimited(retry_after))
            }
            status => Err(ApiError::HttpError(status.as_u16())),
        }
    }
}
```

### 速率限制器

```rust
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{interval, Interval};

pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
    _refill_task: tokio::task::JoinHandle<()>,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window: Duration) -> Self {
        let semaphore = Arc::new(Semaphore::new(max_requests));
        let semaphore_clone = semaphore.clone();

        let refill_task = tokio::spawn(async move {
            let mut interval = interval(window / max_requests as u32);
            loop {
                interval.tick().await;
                if semaphore_clone.available_permits() < max_requests {
                    semaphore_clone.add_permits(1);
                }
            }
        });

        Self {
            semaphore,
            _refill_task: refill_task,
        }
    }

    pub async fn acquire(&self) {
        let _permit = self.semaphore.acquire().await.unwrap();
        // permit 会在 drop 时自动释放
    }
}
```

## 天气 API 集成

### 天气数据结构

```rust
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeatherResponse {
    pub location: Location,
    pub current: CurrentWeather,
    pub forecast: Option<Vec<ForecastDay>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Location {
    pub name: String,
    pub country: String,
    pub region: String,
    pub lat: f64,
    pub lon: f64,
    pub tz_id: String,
    pub localtime: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CurrentWeather {
    pub temp_c: f64,
    pub temp_f: f64,
    pub condition: WeatherCondition,
    pub wind_kph: f64,
    pub humidity: u32,
    pub cloud: u32,
    pub feelslike_c: f64,
    pub vis_km: f64,
    pub uv: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeatherCondition {
    pub text: String,
    pub icon: String,
    pub code: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ForecastDay {
    pub date: String,
    pub day: DayWeather,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DayWeather {
    pub maxtemp_c: f64,
    pub mintemp_c: f64,
    pub condition: WeatherCondition,
    pub avghumidity: u32,
    pub maxwind_kph: f64,
    pub totalprecip_mm: f64,
}
```

### 天气服务实现

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct WeatherService {
    api_client: ApiClient,
    cache: Arc<RwLock<HashMap<String, CachedWeather>>>,
    cache_duration: Duration,
}

#[derive(Clone)]
struct CachedWeather {
    data: WeatherResponse,
    cached_at: std::time::Instant,
}

impl WeatherService {
    pub fn new(api_key: String) -> Result<Self, Box<dyn std::error::Error>> {
        let api_client = ApiClient::new(
            "https://api.weatherapi.com/v1".to_string(),
            Some(api_key),
        )?;

        Ok(Self {
            api_client,
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_duration: Duration::from_secs(600), // 10分钟缓存
        })
    }

    pub async fn get_current_weather(&self, city: &str) -> Result<WeatherResponse, ApiError> {
        let cache_key = format!("current_{}", city.to_lowercase());

        // 检查缓存
        if let Some(cached) = self.get_from_cache(&cache_key).await {
            return Ok(cached);
        }

        // 从 API 获取数据
        let endpoint = format!("current.json?key={}&q={}&aqi=no", 
                             self.api_client.api_key.as_ref().unwrap(), 
                             urlencoding::encode(city));

        let weather_data: WeatherResponse = self.api_client.make_request(&endpoint).await?;

        // 更新缓存
        self.update_cache(cache_key, weather_data.clone()).await;

        Ok(weather_data)
    }

    pub async fn get_forecast(&self, city: &str, days: u8) -> Result<WeatherResponse, ApiError> {
        let cache_key = format!("forecast_{}_{}", city.to_lowercase(), days);

        if let Some(cached) = self.get_from_cache(&cache_key).await {
            return Ok(cached);
        }

        let endpoint = format!("forecast.json?key={}&q={}&days={}&aqi=no&alerts=no",
                             self.api_client.api_key.as_ref().unwrap(),
                             urlencoding::encode(city),
                             days);

        let weather_data: WeatherResponse = self.api_client.make_request(&endpoint).await?;
        self.update_cache(cache_key, weather_data.clone()).await;

        Ok(weather_data)
    }

    async fn get_from_cache(&self, key: &str) -> Option<WeatherResponse> {
        let cache = self.cache.read().await;
        if let Some(cached) = cache.get(key) {
            if cached.cached_at.elapsed() < self.cache_duration {
                return Some(cached.data.clone());
            }
        }
        None
    }

    async fn update_cache(&self, key: String, data: WeatherResponse) {
        let mut cache = self.cache.write().await;
        cache.insert(key, CachedWeather {
            data,
            cached_at: std::time::Instant::now(),
        });

        // 清理过期缓存
        cache.retain(|_, cached| cached.cached_at.elapsed() < self.cache_duration * 2);
    }
}
```

## 翻译服务集成

### 翻译 API 实现

```rust
#[derive(Debug, Deserialize)]
pub struct TranslationResponse {
    pub translations: Vec<Translation>,
}

#[derive(Debug, Deserialize)]
pub struct Translation {
    pub text: String,
    pub detected_source_language: Option<String>,
}

pub struct TranslationService {
    api_client: ApiClient,
}

impl TranslationService {
    pub fn new(api_key: String) -> Result<Self, Box<dyn std::error::Error>> {
        let api_client = ApiClient::new(
            "https://api-free.deepl.com/v2".to_string(),
            Some(api_key),
        )?;

        Ok(Self { api_client })
    }

    pub async fn translate_text(
        &self,
        text: &str,
        target_lang: &str,
        source_lang: Option<&str>,
    ) -> Result<String, ApiError> {
        let mut params = vec![
            ("text", text),
            ("target_lang", target_lang),
        ];

        if let Some(source) = source_lang {
            params.push(("source_lang", source));
        }

        let response: TranslationResponse = self.api_client
            .http_client
            .post(&format!("{}/translate", self.api_client.base_url))
            .header("Authorization", format!("DeepL-Auth-Key {}", 
                   self.api_client.api_key.as_ref().unwrap()))
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        response.translations
            .first()
            .map(|t| t.text.clone())
            .ok_or(ApiError::NoData)
    }

    pub async fn detect_language(&self, text: &str) -> Result<String, ApiError> {
        // 通过翻译到英语来检测语言
        match self.translate_text(text, "EN", None).await {
            Ok(_) => {
                // 这里应该解析 detected_source_language
                // 简化示例直接返回
                Ok("auto".to_string())
            }
            Err(e) => Err(e),
        }
    }
}
```

## 数据库集成

### 用户数据管理

```rust
use sqlx::{PgPool, Row};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct UserData {
    pub user_id: String,
    pub username: String,
    pub guild_id: String,
    pub message_count: i64,
    pub last_active: DateTime<Utc>,
    pub preferences: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

pub struct DatabaseService {
    pool: PgPool,
}

impl DatabaseService {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        
        // 运行迁移
        sqlx::migrate!("./migrations").run(&pool).await?;
        
        Ok(Self { pool })
    }

    pub async fn get_user_data(&self, user_id: &str, guild_id: &str) -> Result<Option<UserData>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT * FROM user_data WHERE user_id = $1 AND guild_id = $2",
            user_id,
            guild_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| UserData {
            user_id: r.user_id,
            username: r.username,
            guild_id: r.guild_id,
            message_count: r.message_count,
            last_active: r.last_active,
            preferences: r.preferences,
            created_at: r.created_at,
        }))
    }

    pub async fn upsert_user(&self, user: &UserData) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO user_data (user_id, username, guild_id, message_count, last_active, preferences)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (user_id, guild_id)
            DO UPDATE SET
                username = EXCLUDED.username,
                message_count = EXCLUDED.message_count,
                last_active = EXCLUDED.last_active,
                preferences = EXCLUDED.preferences
            "#,
            user.user_id,
            user.username,
            user.guild_id,
            user.message_count,
            user.last_active,
            user.preferences
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn increment_message_count(&self, user_id: &str, guild_id: &str) -> Result<i64, sqlx::Error> {
        let row = sqlx::query!(
            "UPDATE user_data SET message_count = message_count + 1, last_active = NOW() WHERE user_id = $1 AND guild_id = $2 RETURNING message_count",
            user_id,
            guild_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.message_count)
    }

    pub async fn get_top_active_users(&self, guild_id: &str, limit: i64) -> Result<Vec<UserData>, sqlx::Error> {
        let rows = sqlx::query!(
            "SELECT * FROM user_data WHERE guild_id = $1 ORDER BY message_count DESC LIMIT $2",
            guild_id,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| UserData {
            user_id: r.user_id,
            username: r.username,
            guild_id: r.guild_id,
            message_count: r.message_count,
            last_active: r.last_active,
            preferences: r.preferences,
            created_at: r.created_at,
        }).collect())
    }
}
```

## 新闻 API 集成

### 新闻服务

```rust
#[derive(Debug, Deserialize, Clone)]
pub struct NewsResponse {
    pub status: String,
    pub articles: Vec<Article>,
    #[serde(rename = "totalResults")]
    pub total_results: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Article {
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    #[serde(rename = "urlToImage")]
    pub url_to_image: Option<String>,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    pub source: ArticleSource,
    pub author: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ArticleSource {
    pub id: Option<String>,
    pub name: String,
}

pub struct NewsService {
    api_client: ApiClient,
}

impl NewsService {
    pub fn new(api_key: String) -> Result<Self, Box<dyn std::error::Error>> {
        let api_client = ApiClient::new(
            "https://newsapi.org/v2".to_string(),
            Some(api_key),
        )?;

        Ok(Self { api_client })
    }

    pub async fn get_top_headlines(&self, country: &str, category: Option<&str>) -> Result<Vec<Article>, ApiError> {
        let mut endpoint = format!("top-headlines?country={}&apiKey={}", 
                                 country, 
                                 self.api_client.api_key.as_ref().unwrap());

        if let Some(cat) = category {
            endpoint.push_str(&format!("&category={}", cat));
        }

        let response: NewsResponse = self.api_client.make_request(&endpoint).await?;
        Ok(response.articles)
    }

    pub async fn search_news(&self, query: &str, page_size: Option<u8>) -> Result<Vec<Article>, ApiError> {
        let page_size = page_size.unwrap_or(10);
        let endpoint = format!("everything?q={}&pageSize={}&apiKey={}", 
                             urlencoding::encode(query),
                             page_size,
                             self.api_client.api_key.as_ref().unwrap());

        let response: NewsResponse = self.api_client.make_request(&endpoint).await?;
        Ok(response.articles)
    }
}
```

## 综合服务管理器

### 服务管理器

```rust
use std::sync::Arc;

pub struct ServiceManager {
    pub weather: Arc<WeatherService>,
    pub translation: Arc<TranslationService>,
    pub database: Arc<DatabaseService>,
    pub news: Arc<NewsService>,
}

impl ServiceManager {
    pub async fn new(config: &ServiceConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let weather = Arc::new(WeatherService::new(config.weather_api_key.clone())?);
        let translation = Arc::new(TranslationService::new(config.deepl_api_key.clone())?);
        let database = Arc::new(DatabaseService::new(&config.database_url).await?);
        let news = Arc::new(NewsService::new(config.news_api_key.clone())?);

        Ok(Self {
            weather,
            translation,
            database,
            news,
        })
    }
}

pub struct ServiceConfig {
    pub weather_api_key: String,
    pub deepl_api_key: String,
    pub database_url: String,
    pub news_api_key: String,
}

impl ServiceConfig {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            weather_api_key: std::env::var("WEATHER_API_KEY")?,
            deepl_api_key: std::env::var("DEEPL_API_KEY")?,
            database_url: std::env::var("DATABASE_URL")?,
            news_api_key: std::env::var("NEWS_API_KEY")?,
        })
    }
}
```

## 机器人事件处理器集成

### 集成事件处理器

```rust
use botrs::{Context, EventHandler, Message, Ready, MessageParams, Embed};
use tracing::{info, warn, error};

pub struct ApiIntegratedBot {
    services: Arc<ServiceManager>,
}

impl ApiIntegratedBot {
    pub fn new(services: Arc<ServiceManager>) -> Self {
        Self { services }
    }

    async fn handle_weather_command(&self, ctx: &Context, message: &Message, city: &str) {
        match self.services.weather.get_current_weather(city).await {
            Ok(weather) => {
                let embed = self.create_weather_embed(&weather);
                let params = MessageParams::new_embed(embed);
                
                if let Err(e) = ctx.api.post_message_with_params(&ctx.token, &message.channel_id, params).await {
                    warn!("发送天气信息失败: {}", e);
                }
            }
            Err(e) => {
                error!("获取天气信息失败: {}", e);
                let error_msg = "抱歉，无法获取天气信息，请稍后重试。";
                if let Err(e) = message.reply(&ctx.api, &ctx.token, error_msg).await {
                    warn!("发送错误消息失败: {}", e);
                }
            }
        }
    }

    fn create_weather_embed(&self, weather: &WeatherResponse) -> Embed {
        let condition_emoji = match weather.current.condition.code {
            1000 => "☀️", // Sunny
            1003 => "⛅", // Partly cloudy
            1006 => "☁️", // Cloudy
            1009 => "☁️", // Overcast
            1030 => "🌫️", // Mist
            1063..=1201 => "🌧️", // Rain
            1210..=1282 => "❄️", // Snow
            _ => "🌤️",
        };

        Embed::new()
            .title(&format!("{} {} 天气", condition_emoji, weather.location.name))
            .description(&weather.current.condition.text)
            .color(0x3498db)
            .field("🌡️ 温度", &format!("{}°C", weather.current.temp_c), true)
            .field("🌡️ 体感温度", &format!("{}°C", weather.current.feelslike_c), true)
            .field("💧 湿度", &format!("{}%", weather.current.humidity), true)
            .field("💨 风速", &format!("{} km/h", weather.current.wind_kph), true)
            .field("☁️ 云量", &format!("{}%", weather.current.cloud), true)
            .field("👁️ 能见度", &format!("{} km", weather.current.vis_km), true)
            .thumbnail(&format!("https:{}", weather.current.condition.icon))
            .footer("数据来源: WeatherAPI", None)
            .timestamp(chrono::Utc::now())
    }

    async fn handle_translate_command(&self, ctx: &Context, message: &Message, args: &[&str]) {
        if args.len() < 2 {
            let _ = message.reply(&ctx.api, &ctx.token, "用法: !translate <目标语言> <要翻译的文本>").await;
            return;
        }

        let target_lang = args[0];
        let text = args[1..].join(" ");

        match self.services.translation.translate_text(&text, target_lang, None).await {
            Ok(translated) => {
                let response = format!("翻译结果:\n原文: {}\n译文: {}", text, translated);
                if let Err(e) = message.reply(&ctx.api, &ctx.token, &response).await {
                    warn!("发送翻译结果失败: {}", e);
                }
            }
            Err(e) => {
                error!("翻译失败: {}", e);
                let _ = message.reply(&ctx.api, &ctx.token, "翻译失败，请检查语言代码和文本内容").await;
            }
        }
    }

    async fn handle_news_command(&self, ctx: &Context, message: &Message, query: Option<&str>) {
        let articles = match query {
            Some(q) => self.services.news.search_news(q, Some(5)).await,
            None => self.services.news.get_top_headlines("cn", None).await,
        };

        match articles {
            Ok(articles) => {
                if articles.is_empty() {
                    let _ = message.reply(&ctx.api, &ctx.token, "没有找到相关新闻").await;
                    return;
                }

                let embed = self.create_news_embed(&articles[0..3.min(articles.len())]);
                let params = MessageParams::new_embed(embed);
                
                if let Err(e) = ctx.api.post_message_with_params(&ctx.token, &message.channel_id, params).await {
                    warn!("发送新闻信息失败: {}", e);
                }
            }
            Err(e) => {
                error!("获取新闻失败: {}", e);
                let _ = message.reply(&ctx.api, &ctx.token, "获取新闻失败，请稍后重试").await;
            }
        }
    }

    fn create_news_embed(&self, articles: &[Article]) -> Embed {
        let mut embed = Embed::new()
            .title("📰 最新新闻")
            .color(0xe74c3c);

        for (i, article) in articles.iter().enumerate() {
            let title = if article.title.len() > 100 {
                format!("{}...", &article.title[..97])
            } else {
                article.title.clone()
            };

            let description = article.description
                .as_ref()
                .map(|d| if d.len() > 200 { format!("{}...", &d[..197]) } else { d.clone() })
                .unwrap_or_else(|| "无描述".to_string());

            embed = embed.field(
                &format!("{}. {}", i + 1, title),
                &format!("{}\n[阅读更多]({})", description, article.url),
                false,
            );
        }

        embed.footer("新闻来源: NewsAPI", None)
            .timestamp(chrono::Utc::now())
    }

    async fn update_user_activity(&self, message: &Message) {
        if let Some(author) = &message.author {
            if let Some(guild_id) = &message.guild_id {
                if let Err(e) = self.services.database.increment_message_count(&author.id, guild_id).await {
                    warn!("更新用户活动失败: {}", e);
                }
            }
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for ApiIntegratedBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("API 集成机器人已就绪: {}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        // 更新用户活动
        self.update_user_activity(&message).await;

        let content = match &message.content {
            Some(content) => content.trim(),
            None => return,
        };

        let args: Vec<&str> = content.split_whitespace().collect();
        if args.is_empty() {
            return;
        }

        match args[0] {
            "!weather" | "!天气" => {
                if args.len() > 1 {
                    let city = args[1..].join(" ");
                    self.handle_weather_command(&ctx, &message, &city).await;
                } else {
                    let _ = message.reply(&ctx.api, &ctx.token, "用法: !天气 <城市名称>").await;
                }
            }
            "!translate" | "!翻译" => {
                if args.len() > 2 {
                    self.handle_translate_command(&ctx, &message, &args[1..]).await;
                } else {
                    let _ = message.reply(&ctx.api, &ctx.token, "用法: !翻译 <目标语言> <文本>").await;
                }
            }
            "!news" | "!新闻" => {
                let query = if args.len() > 1 {
                    Some(args[1..].join(" "))
                } else {
                    None
                };
                self.handle_news_command(&ctx, &message, query.as_deref()).await;
            }
            "!stats" | "!统计" => {
                if let (Some(author), Some(guild_id)) = (&message.author, &message.guild_id) {
                    match self.services.database.get_user_data(&author.id, guild_id).await {
                        Ok(Some(user_data)) => {
                            let stats_msg = format!(
                                "📊 用户统计\n用户: {}\n消息数: {}\n最后活跃: {}",
                                user_data.username,
                                user_data.message_count,
                                user_data.last_active.format("%Y-%m-%d %H:%M")
                            );
                            let _ = message.reply(&ctx.api, &ctx.token, &stats_msg).await;
                        }
                        Ok(None) => {
                            let _ = message.reply(&ctx.api, &ctx.token, "未找到用户数据").await;
                        }
                        Err(e) => {
                            error!("查询用户数据失败: {}", e);
                            let _ = message.reply(&ctx.api, &ctx.token, "查询统计信息失败").await;
                        }
                    }
                }
            }
            "!top" | "!排行" => {
                if let Some(guild_id) = &message.guild_id {
                    match self.services.database.get_top_active_users(guild_id, 10).await {
                        Ok(users) => {
                            let mut leaderboard = "🏆 活跃用户排行榜\n".to_string();
                            for (i, user) in users.iter().enumerate() {
                                leaderboard.push_str(&format!(
                                    "{}. {} - {} 条消息\n",
                                    i + 1,
                                    user.username,
                                    user.message_count
                                ));
                            }
                            let _ = message.reply(&ctx.api, &ctx.token, &leaderboard).await;
                        }
                        Err(e) => {
                            error!("获取排行榜失败: {}", e);
                            let _ = message.reply(&ctx.api, &ctx.token, "获取排行榜失败").await;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
```

## 错误处理

### API 错误类型定义

```rust
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("网络请求错误: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("HTTP 错误: 状态码 {0}")]
    HttpError(u16),
    
    #[error("速率限制: {0} 秒后重试")]
    RateLimited(u64),
    
    #[error("API 密钥无效")]
    InvalidApiKey,
    
    #[error("数据解析错误: {0}")]
    ParseError(#[from] serde_json::Error),
    
    #[error("数据库错误: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("没有数据返回")]
    NoData,
    
    #[error("服务不可用")]
    ServiceUnavailable,
    
    #[error("自定义错误: {0}")]
    Custom(String),
}
```

### 错误恢复策略

```rust
use std::time::Duration;
use tokio::time::sleep;

pub struct ErrorRecoveryManager;

impl ErrorRecoveryManager {
    pub async fn handle_api_error<T, F, Fut>(
        operation: F,
        max_retries: usize,
        operation_name: &str,
    ) -> Result<T, ApiError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, ApiError>>,
    {
        let mut last_error = None;
        
        for attempt in 1..=max_retries {
            match operation().await {
                Ok(result) => {
                    if attempt > 1 {
                        info!("{} 在第 {} 次尝试后成功", operation_name, attempt);
                    }
                    return Ok(result);
                }
                Err(error) => {
                    warn!("{} 第 {} 次尝试失败: {}", operation_name, attempt, error);
                    
                    match &error {
                        ApiError::RateLimited(retry_after) => {
                            if attempt < max_retries {
                                info!("等待 {} 秒后重试", retry_after);
                                sleep(Duration::from_secs(*retry_after)).await;
                                continue;
                            }
                        }
                        ApiError::Network(_) => {
                            if attempt < max_retries {
                                let delay = std::cmp::min(2_u64.pow(attempt as u32), 30);
                                info!("网络错误，{} 秒后重试", delay);
                                sleep(Duration::from_secs(delay)).await;
                                continue;
                            }
                        }
                        ApiError::ServiceUnavailable => {
                            if attempt < max_retries {
                                let delay = 5 * attempt as u64;
                                info!("服务不可用，{} 秒后重试", delay);
                                sleep(Duration::from_secs(delay)).await;
                                continue;
                            }
                        }
                        ApiError::InvalidApiKey | ApiError::ParseError(_) => {
                            // 这些错误不应该重试
                            return Err(error);
                        }
                        _ => {}
                    }
                    
                    last_error = Some(error);
                }
            }
        }
        
        Err(last_error.unwrap_or_else(|| ApiError::Custom("未知错误".to_string())))
    }
}
```

## 完整示例程序

```rust
use botrs::{Client, Intents, Token};
use std::sync::Arc;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter("botrs=debug,api_integration=info")
        .init();
    
    info!("启动 API 集成示例机器人");
    
    // 加载配置
    let bot_token = Token::from_env()?;
    bot_token.validate()?;
    
    let service_config = ServiceConfig::from_env()?;
    let services = Arc::new(ServiceManager::new(&service_config).await?);
    
    // 配置 Intent
    let intents = Intents::default()
        .with_public_guild_messages()
        .with_direct_message()
        .with_guilds();
    
    // 创建事件处理器
    let handler = ApiIntegratedBot::new(services);
    
    // 创建并启动客户端
    let mut client = Client::new(bot_token, intents, handler, false)?;
    
    info!("API 集成机器人启动中...");
    client.start().await?;
    
    info!("API 集成机器人已停止");
    Ok(())
}
```

## 最佳实践

### API 安全
1. **密钥管理**: 使用环境变量存储 API 密钥
2. **速率限制**: 实现智能速率限制避免超额使用
3. **错误处理**: 对不同类型的错误采用合适的处理策略
4. **数据验证**: 验证从外部 API 获取的数据

### 性能优化
1. **缓存策略**: 对频繁访问的数据实现适当缓存
2. **连接池**: 复用 HTTP 连接减少开销
3. **并发控制**: 避免同时发起过多请求
4. **超时设置**: 设置合理的请求超时时间

### 监控和调试
1. **日志记录**: 记录 API 调用和错误信息
2. **指标收集**: 监控 API 使用量和响应时间
3. **健康检查**: 定期检查外部服务可用性
4. **告警机制**: 在服务异常时及时通知

通过合理的 API 集成策略，您可以为机器人添加丰富的外部服务功能，提供更好的用户体验。

## 另请参阅

- [错误恢复示例](/zh/examples/error-recovery.md) - 错误处理和恢复机制
- [事件处理示例](/zh/examples/event-handling.md) - 事件系统集成
- [API 客户端使用](/zh/guide/api-client.md) - API 客户端使用指南
- [错误处理指南](/zh/guide/error-handling.md) - 错误处理最佳实践