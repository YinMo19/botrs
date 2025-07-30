# API Integration Example

This example demonstrates how to integrate external APIs and services with your QQ Guild bot using BotRS.

## Overview

API integration allows your bot to fetch data from external services, interact with databases, and provide rich functionality beyond basic messaging. This example shows various integration patterns including REST APIs, webhooks, and third-party services.

## Basic API Client Setup

```rust
use botrs::{
    Client, Context, EventHandler, Intents, Message, Ready, Token, BotError
};
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;

struct ApiIntegrationBot {
    http_client: reqwest::Client,
    api_keys: HashMap<String, String>,
}

impl ApiIntegrationBot {
    pub fn new() -> Self {
        let mut api_keys = HashMap::new();

        // Load API keys from environment
        if let Ok(weather_key) = std::env::var("WEATHER_API_KEY") {
            api_keys.insert("weather".to_string(), weather_key);
        }

        if let Ok(news_key) = std::env::var("NEWS_API_KEY") {
            api_keys.insert("news".to_string(), news_key);
        }

        Self {
            http_client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .user_agent("BotRS/0.2.5")
                .build()
                .expect("Failed to create HTTP client"),
            api_keys,
        }
    }
}

#[async_trait]
impl EventHandler for ApiIntegrationBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("API Integration bot ready: {}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            let parts: Vec<&str> = content.trim().split_whitespace().collect();

            match parts.get(0) {
                Some(&"!weather") => {
                    if let Some(city) = parts.get(1) {
                        self.handle_weather_request(&ctx, &message, city).await;
                    } else {
                        let _ = message.reply(&ctx.api, &ctx.token, "Usage: !weather <city>").await;
                    }
                }
                Some(&"!news") => {
                    self.handle_news_request(&ctx, &message).await;
                }
                Some(&"!translate") => {
                    if parts.len() >= 3 {
                        let lang = parts[1];
                        let text = parts[2..].join(" ");
                        self.handle_translation_request(&ctx, &message, lang, &text).await;
                    } else {
                        let _ = message.reply(&ctx.api, &ctx.token, "Usage: !translate <language> <text>").await;
                    }
                }
                Some(&"!quote") => {
                    self.handle_quote_request(&ctx, &message).await;
                }
                Some(&"!crypto") => {
                    if let Some(symbol) = parts.get(1) {
                        self.handle_crypto_request(&ctx, &message, symbol).await;
                    } else {
                        let _ = message.reply(&ctx.api, &ctx.token, "Usage: !crypto <symbol>").await;
                    }
                }
                _ => {}
            }
        }
    }
}
```

## Weather API Integration

```rust
#[derive(Deserialize)]
struct WeatherResponse {
    name: String,
    main: WeatherMain,
    weather: Vec<WeatherCondition>,
    wind: Option<WeatherWind>,
}

#[derive(Deserialize)]
struct WeatherMain {
    temp: f64,
    feels_like: f64,
    humidity: u32,
    pressure: u32,
}

#[derive(Deserialize)]
struct WeatherCondition {
    main: String,
    description: String,
}

#[derive(Deserialize)]
struct WeatherWind {
    speed: f64,
}

impl ApiIntegrationBot {
    async fn handle_weather_request(&self, ctx: &Context, message: &Message, city: &str) {
        if let Some(api_key) = self.api_keys.get("weather") {
            match self.fetch_weather(city, api_key).await {
                Ok(weather) => {
                    let response = self.format_weather_response(&weather);
                    let _ = message.reply(&ctx.api, &ctx.token, &response).await;
                }
                Err(e) => {
                    eprintln!("Weather API error: {}", e);
                    let _ = message.reply(&ctx.api, &ctx.token, "❌ Failed to fetch weather data").await;
                }
            }
        } else {
            let _ = message.reply(&ctx.api, &ctx.token, "⚠️ Weather API key not configured").await;
        }
    }

    async fn fetch_weather(&self, city: &str, api_key: &str) -> Result<WeatherResponse, BotError> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
            urlencoding::encode(city),
            api_key
        );

        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| BotError::Network(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(BotError::Api(format!("Weather API returned status: {}", response.status())));
        }

        let weather: WeatherResponse = response
            .json()
            .await
            .map_err(|e| BotError::Serialization(format!("Failed to parse JSON: {}", e)))?;

        Ok(weather)
    }

    fn format_weather_response(&self, weather: &WeatherResponse) -> String {
        let condition = weather.weather.first()
            .map(|w| &w.description)
            .unwrap_or("Unknown");

        let wind_info = weather.wind
            .as_ref()
            .map(|w| format!("💨 Wind: {:.1} m/s", w.speed))
            .unwrap_or_default();

        format!(
            "🌤️ **Weather in {}**\n\n\
             🌡️ Temperature: {:.1}°C (feels like {:.1}°C)\n\
             ☁️ Condition: {}\n\
             💧 Humidity: {}%\n\
             📊 Pressure: {} hPa\n\
             {}",
            weather.name,
            weather.main.temp,
            weather.main.feels_like,
            condition,
            weather.main.humidity,
            weather.main.pressure,
            wind_info
        )
    }
}
```

## News API Integration

```rust
#[derive(Deserialize)]
struct NewsResponse {
    articles: Vec<NewsArticle>,
}

#[derive(Deserialize)]
struct NewsArticle {
    title: String,
    description: Option<String>,
    url: String,
    #[serde(rename = "publishedAt")]
    published_at: String,
    source: NewsSource,
}

#[derive(Deserialize)]
struct NewsSource {
    name: String,
}

impl ApiIntegrationBot {
    async fn handle_news_request(&self, ctx: &Context, message: &Message) {
        if let Some(api_key) = self.api_keys.get("news") {
            match self.fetch_latest_news(api_key).await {
                Ok(articles) => {
                    let response = self.format_news_response(&articles);
                    let _ = message.reply(&ctx.api, &ctx.token, &response).await;
                }
                Err(e) => {
                    eprintln!("News API error: {}", e);
                    let _ = message.reply(&ctx.api, &ctx.token, "❌ Failed to fetch news").await;
                }
            }
        } else {
            let _ = message.reply(&ctx.api, &ctx.token, "⚠️ News API key not configured").await;
        }
    }

    async fn fetch_latest_news(&self, api_key: &str) -> Result<Vec<NewsArticle>, BotError> {
        let url = format!(
            "https://newsapi.org/v2/top-headlines?country=us&pageSize=5&apiKey={}",
            api_key
        );

        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| BotError::Network(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(BotError::Api(format!("News API returned status: {}", response.status())));
        }

        let news_response: NewsResponse = response
            .json()
            .await
            .map_err(|e| BotError::Serialization(format!("Failed to parse JSON: {}", e)))?;

        Ok(news_response.articles)
    }

    fn format_news_response(&self, articles: &[NewsArticle]) -> String {
        let mut response = String::from("📰 **Latest News**\n\n");

        for (index, article) in articles.iter().take(3).enumerate() {
            response.push_str(&format!(
                "**{}. {}**\n{}\n*Source: {}*\n[Read more]({})\n\n",
                index + 1,
                article.title,
                article.description.as_deref().unwrap_or("No description available"),
                article.source.name,
                article.url
            ));
        }

        response
    }
}
```

## Translation API Integration

```rust
#[derive(Serialize)]
struct TranslationRequest {
    q: String,
    target: String,
    source: String,
}

#[derive(Deserialize)]
struct TranslationResponse {
    data: TranslationData,
}

#[derive(Deserialize)]
struct TranslationData {
    translations: Vec<Translation>,
}

#[derive(Deserialize)]
struct Translation {
    #[serde(rename = "translatedText")]
    translated_text: String,
    #[serde(rename = "detectedSourceLanguage")]
    detected_source_language: Option<String>,
}

impl ApiIntegrationBot {
    async fn handle_translation_request(&self, ctx: &Context, message: &Message, target_lang: &str, text: &str) {
        match self.translate_text(text, target_lang).await {
            Ok(translation) => {
                let response = format!(
                    "🌐 **Translation**\n\n\
                     **Original:** {}\n\
                     **Translated ({}):** {}",
                    text,
                    target_lang,
                    translation
                );
                let _ = message.reply(&ctx.api, &ctx.token, &response).await;
            }
            Err(e) => {
                eprintln!("Translation error: {}", e);
                let _ = message.reply(&ctx.api, &ctx.token, "❌ Translation failed").await;
            }
        }
    }

    async fn translate_text(&self, text: &str, target_lang: &str) -> Result<String, BotError> {
        // Using a free translation API (LibreTranslate)
        let url = "https://libretranslate.de/translate";

        let request_body = serde_json::json!({
            "q": text,
            "source": "auto",
            "target": target_lang,
            "format": "text"
        });

        let response = self.http_client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| BotError::Network(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(BotError::Api(format!("Translation API returned status: {}", response.status())));
        }

        let translation_response: serde_json::Value = response
            .json()
            .await
            .map_err(|e| BotError::Serialization(format!("Failed to parse JSON: {}", e)))?;

        let translated_text = translation_response["translatedText"]
            .as_str()
            .ok_or_else(|| BotError::Serialization("Missing translatedText field".to_string()))?;

        Ok(translated_text.to_string())
    }
}
```

## Cryptocurrency API Integration

```rust
#[derive(Deserialize)]
struct CryptoResponse {
    #[serde(flatten)]
    coins: HashMap<String, CoinData>,
}

#[derive(Deserialize)]
struct CoinData {
    usd: f64,
    usd_24h_change: f64,
}

impl ApiIntegrationBot {
    async fn handle_crypto_request(&self, ctx: &Context, message: &Message, symbol: &str) {
        match self.fetch_crypto_price(symbol).await {
            Ok((price, change)) => {
                let change_emoji = if change >= 0.0 { "📈" } else { "📉" };
                let change_sign = if change >= 0.0 { "+" } else { "" };

                let response = format!(
                    "💰 **{} Price**\n\n\
                     💵 Current: ${:.2}\n\
                     {} 24h Change: {}{:.2}%",
                    symbol.to_uppercase(),
                    price,
                    change_emoji,
                    change_sign,
                    change
                );
                let _ = message.reply(&ctx.api, &ctx.token, &response).await;
            }
            Err(e) => {
                eprintln!("Crypto API error: {}", e);
                let _ = message.reply(&ctx.api, &ctx.token, "❌ Failed to fetch cryptocurrency data").await;
            }
        }
    }

    async fn fetch_crypto_price(&self, symbol: &str) -> Result<(f64, f64), BotError> {
        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd&include_24hr_change=true",
            symbol.to_lowercase()
        );

        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| BotError::Network(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(BotError::Api(format!("Crypto API returned status: {}", response.status())));
        }

        let crypto_response: CryptoResponse = response
            .json()
            .await
            .map_err(|e| BotError::Serialization(format!("Failed to parse JSON: {}", e)))?;

        let coin_data = crypto_response.coins
            .get(&symbol.to_lowercase())
            .ok_or_else(|| BotError::InvalidInput(format!("Cryptocurrency '{}' not found", symbol)))?;

        Ok((coin_data.usd, coin_data.usd_24h_change))
    }
}
```

## Quote API Integration

```rust
#[derive(Deserialize)]
struct QuoteResponse {
    content: String,
    author: String,
}

impl ApiIntegrationBot {
    async fn handle_quote_request(&self, ctx: &Context, message: &Message) {
        match self.fetch_random_quote().await {
            Ok(quote) => {
                let response = format!(
                    "💭 **Quote of the Moment**\n\n\
                     \"{}\"\n\n\
                     *— {}*",
                    quote.content,
                    quote.author
                );
                let _ = message.reply(&ctx.api, &ctx.token, &response).await;
            }
            Err(e) => {
                eprintln!("Quote API error: {}", e);
                let _ = message.reply(&ctx.api, &ctx.token, "❌ Failed to fetch quote").await;
            }
        }
    }

    async fn fetch_random_quote(&self) -> Result<QuoteResponse, BotError> {
        let url = "https://api.quotable.io/random";

        let response = self.http_client
            .get(url)
            .send()
            .await
            .map_err(|e| BotError::Network(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(BotError::Api(format!("Quote API returned status: {}", response.status())));
        }

        let quote: QuoteResponse = response
            .json()
            .await
            .map_err(|e| BotError::Serialization(format!("Failed to parse JSON: {}", e)))?;

        Ok(quote)
    }
}
```

## Database Integration

```rust
use sqlx::{Pool, Sqlite, SqlitePool};

struct DatabaseBot {
    http_client: reqwest::Client,
    db_pool: Option<SqlitePool>,
}

impl DatabaseBot {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db_pool = if let Ok(database_url) = std::env::var("DATABASE_URL") {
            Some(SqlitePool::connect(&database_url).await?)
        } else {
            None
        };

        Ok(Self {
            http_client: reqwest::Client::new(),
            db_pool,
        })
    }

    async fn save_user_preference(&self, user_id: &str, preference: &str, value: &str) -> Result<(), BotError> {
        if let Some(pool) = &self.db_pool {
            sqlx::query!(
                "INSERT OR REPLACE INTO user_preferences (user_id, preference, value) VALUES (?, ?, ?)",
                user_id,
                preference,
                value
            )
            .execute(pool)
            .await
            .map_err(|e| BotError::InternalError(format!("Database error: {}", e)))?;
        }
        Ok(())
    }

    async fn get_user_preference(&self, user_id: &str, preference: &str) -> Result<Option<String>, BotError> {
        if let Some(pool) = &self.db_pool {
            let row = sqlx::query!(
                "SELECT value FROM user_preferences WHERE user_id = ? AND preference = ?",
                user_id,
                preference
            )
            .fetch_optional(pool)
            .await
            .map_err(|e| BotError::InternalError(format!("Database error: {}", e)))?;

            Ok(row.map(|r| r.value))
        } else {
            Ok(None)
        }
    }
}
```

## Rate Limiting and Caching

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct CacheEntry<T> {
    data: T,
    expires_at: Instant,
}

struct ApiCache<T> {
    entries: Arc<Mutex<HashMap<String, CacheEntry<T>>>>,
    ttl: Duration,
}

impl<T: Clone> ApiCache<T> {
    fn new(ttl: Duration) -> Self {
        Self {
            entries: Arc::new(Mutex::new(HashMap::new())),
            ttl,
        }
    }

    async fn get(&self, key: &str) -> Option<T> {
        let mut entries = self.entries.lock().await;

        if let Some(entry) = entries.get(key) {
            if Instant::now() < entry.expires_at {
                return Some(entry.data.clone());
            } else {
                entries.remove(key);
            }
        }

        None
    }

    async fn set(&self, key: String, data: T) {
        let mut entries = self.entries.lock().await;
        entries.insert(key, CacheEntry {
            data,
            expires_at: Instant::now() + self.ttl,
        });
    }
}

struct CachedApiBot {
    http_client: reqwest::Client,
    weather_cache: ApiCache<WeatherResponse>,
    rate_limiter: Arc<Mutex<HashMap<String, Instant>>>,
}

impl CachedApiBot {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            weather_cache: ApiCache::new(Duration::from_secs(600)), // 10 minutes cache
            rate_limiter: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn check_rate_limit(&self, user_id: &str) -> bool {
        let mut limiter = self.rate_limiter.lock().await;
        let now = Instant::now();

        if let Some(&last_request) = limiter.get(user_id) {
            if now.duration_since(last_request) < Duration::from_secs(5) {
                return false; // Rate limited
            }
        }

        limiter.insert(user_id.to_string(), now);
        true
    }

    async fn get_cached_weather(&self, city: &str, api_key: &str) -> Result<WeatherResponse, BotError> {
        let cache_key = format!("weather_{}", city.to_lowercase());

        // Check cache first
        if let Some(cached_weather) = self.weather_cache.get(&cache_key).await {
            return Ok(cached_weather);
        }

        // Fetch from API
        let weather = self.fetch_weather_from_api(city, api_key).await?;

        // Cache the result
        self.weather_cache.set(cache_key, weather.clone()).await;

        Ok(weather)
    }

    async fn fetch_weather_from_api(&self, city: &str, api_key: &str) -> Result<WeatherResponse, BotError> {
        // Implementation similar to previous weather fetching function
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
            urlencoding::encode(city),
            api_key
        );

        let response = self.http_client.get(&url).send().await
            .map_err(|e| BotError::Network(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(BotError::Api(format!("Weather API returned status: {}", response.status())));
        }

        response.json().await
            .map_err(|e| BotError::Serialization(format!("Failed to parse JSON: {}", e)))
    }
}
```

## Webhook Integration

```rust
use warp::Filter;
use tokio::sync::mpsc;

struct WebhookBot {
    webhook_sender: mpsc::UnboundedSender<WebhookEvent>,
}

#[derive(Debug, Clone)]
struct WebhookEvent {
    event_type: String,
    data: serde_json::Value,
}

impl WebhookBot {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<WebhookEvent>) {
        let (tx, rx) = mpsc::unbounded_channel();

        (Self {
            webhook_sender: tx,
        }, rx)
    }

    pub async fn start_webhook_server(&self, port: u16) {
        let sender = self.webhook_sender.clone();

        let webhook = warp::path("webhook")
            .and(warp::post())
            .and(warp::body::json())
            .map(move |body: serde_json::Value| {
                let event = WebhookEvent {
                    event_type: "generic".to_string(),
                    data: body,
                };

                if sender.send(event).is_err() {
                    eprintln!("Failed to send webhook event");
                }

                warp::reply::with_status("OK", warp::http::StatusCode::OK)
            });

        warp::serve(webhook)
            .run(([127, 0, 0, 1], port))
            .await;
    }

    pub async fn process_webhook_events(&self, mut rx: mpsc::UnboundedReceiver<WebhookEvent>, ctx: Context) {
        while let Some(event) = rx.recv().await {
            println!("Received webhook event: {:?}", event);

            // Process different webhook events
            match event.event_type.as_str() {
                "github_push" => self.handle_github_push(&ctx, &event.data).await,
                "ci_build" => self.handle_ci_build(&ctx, &event.data).await,
                _ => println!("Unknown webhook event type: {}", event.event_type),
            }
        }
    }

    async fn handle_github_push(&self, ctx: &Context, data: &serde_json::Value) {
        if let Some(repo_name) = data["repository"]["name"].as_str() {
            if let Some(pusher) = data["pusher"]["name"].as_str() {
                let message = format!("🔄 **Git Push**\n\n{} pushed to {}", pusher, repo_name);
                // Send to appropriate channel
                // ctx.send_message(channel_id, &MessageParams::new_text(&message)).await;
            }
        }
    }

    async fn handle_ci_build(&self, ctx: &Context, data: &serde_json::Value) {
        if let Some(status) = data["status"].as_str() {
            let emoji = match status {
                "success" => "✅",
                "failure" => "❌",
                "pending" => "⏳",
                _ => "🔄",
            };

            let message = format!("{} **Build Status:** {}", emoji, status);
            // Send to appropriate channel
        }
    }
}
```

## Main Application with All Integrations

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("botrs=debug,api_integration=info")
        .init();

    println!("🚀 Starting API integration bot...");

    // Get credentials
    let app_id = std::env::var("QQ_BOT_APP_ID")
        .expect("QQ_BOT_APP_ID environment variable required");
    let secret = std::env::var("QQ_BOT_SECRET")
        .expect("QQ_BOT_SECRET environment variable required");

    // Create token
    let token = Token::new(app_id, secret);
    token.validate()?;

    // Set up intents
    let intents = Intents::default()
        .with_public_guild_messages()
        .with_direct_message()
        .with_guilds();

    // Create bot with API integrations
    let handler = ApiIntegrationBot::new();
    let mut client = Client::new(token, intents, handler, false)?;

    println!("🌐 API integration bot starting...");
    client.start().await?;

    Ok(())
}
```

## Usage Examples

### API Commands

```
# Weather information
!weather London
!weather Tokyo

# Latest news
!news

# Translation
!translate es Hello world
!translate fr How are you?

# Cryptocurrency prices
!crypto bitcoin
!crypto ethereum

# Random quote
!quote
```

### Environment Variables

```bash
# API Keys (optional - features disabled if not provided)
export WEATHER_API_KEY="your_openweather_api_key"
export NEWS_API_KEY="your_news_api_key"

# Database
export DATABASE_URL="sqlite:bot.db"

# Bot credentials
export QQ_BOT_APP_ID="your_app_id"
export QQ_BOT_SECRET="your_secret"
```

## Best Practices

### API Integration

1. **Error Handling**: Always handle API failures gracefully
2. **Rate Limiting**: Respect API rate limits and implement backoff
3. **Caching**: Cache responses to reduce API calls and improve performance
4. **Timeouts**: Set reasonable timeouts for HTTP requests
5. **Validation**: Validate API responses before processing
6. **Fallbacks**: Provide fallback responses when APIs are unavailable

### Security

1. **API Keys**: Store API keys securely in environment variables
2. **Input Validation**: Validate user input before sending to APIs
3. **HTTPS**: Always use HTTPS for API communications
4. **Secrets Management**: Use proper secrets management in production
5. **Access Control**: Limit API access based on user permissions

### Performance

1. **Connection Pooling**: Reuse HTTP connections
2. **Async Operations**: Use async/await for non-blocking operations
3. **Batch Requests**: Batch multiple API calls when possible
4. **Lazy Loading**: Load data only when needed
5. **Monitoring**: Monitor API response times and error rates

## Common Integration Patterns

- **Command-Based APIs**: Trigger API calls based on bot commands
- **Scheduled Updates**: Periodically fetch data from APIs
- **Webhook Processing**: Handle incoming webhook events
- **Real-time Data**: Stream real-time data from APIs
- **Multi-step Workflows**: Chain multiple API calls together

## Troubleshooting

### Common Issues

1. **API Key Issues**: Verify API keys are correct and active
2. **Rate Limiting**: Implement proper rate limiting and backoff
3. **Network Timeouts**: Increase timeout values or implement retry logic
4. **JSON Parsing**: Validate API response format before parsing
5. **CORS Issues**: Not applicable for server-side bot applications

### Debugging Tips

1. **Logging**: Add comprehensive logging for API requests/responses
2. **Error Messages**: Provide detailed error messages for debugging
3. **Health Checks**: Implement API health checking
4. **Monitoring**: Set up monitoring for API integrations
5. **Testing**: Test with different API responses and edge cases

## See Also

- [Error Recovery](./error-recovery.md) - Handling API failures gracefully
- [Command Handler](./command-handler.md) - Structured command processing
- [Getting Started](./getting-started.md) - Basic bot setup
- [Event Handling](./event-handling.md) - Processing bot events
