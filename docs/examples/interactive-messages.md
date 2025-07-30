# Interactive Messages Example

This example demonstrates how to create interactive messages with buttons, keyboards, and rich embeds using BotRS.

## Overview

Interactive messages allow users to interact with your bot through buttons, select menus, and other UI components. This creates a more engaging user experience compared to text-only interactions.

## Basic Interactive Components

### Simple Button Example

```rust
use botrs::{
    Client, Context, EventHandler, Intents, Message, Ready, Token, BotError,
    models::message::{
        Keyboard, KeyboardContent, KeyboardRow, KeyboardButton,
        KeyboardButtonRenderData, KeyboardButtonAction, MessageParams
    }
};

struct InteractiveBot;

#[async_trait::async_trait]
impl EventHandler for InteractiveBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Interactive bot ready: {}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            match content.trim() {
                "!button" => {
                    if let Err(e) = self.send_button_message(&ctx, &message).await {
                        eprintln!("Failed to send button message: {}", e);
                    }
                }
                "!poll" => {
                    if let Err(e) = self.send_poll_message(&ctx, &message).await {
                        eprintln!("Failed to send poll: {}", e);
                    }
                }
                "!menu" => {
                    if let Err(e) = self.send_menu_message(&ctx, &message).await {
                        eprintln!("Failed to send menu: {}", e);
                    }
                }
                _ => {}
            }
        }
    }
}

impl InteractiveBot {
    async fn send_button_message(&self, ctx: &Context, message: &Message) -> Result<(), BotError> {
        let keyboard = Keyboard {
            content: Some(KeyboardContent {
                rows: vec![
                    KeyboardRow {
                        buttons: vec![
                            KeyboardButton {
                                id: Some("btn_action".to_string()),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "Click Me! 👆".to_string(),
                                    visited_label: "Clicked! ✅".to_string(),
                                    style: Some(1), // Primary style
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2), // Callback action
                                    permission: None,
                                    click_limit: None,
                                    data: Some("button_clicked".to_string()),
                                    reply: None,
                                    enter: Some(true),
                                }),
                            },
                        ],
                    },
                ],
            }),
        };

        let params = MessageParams {
            content: Some("Here's an interactive button:".to_string()),
            keyboard: Some(keyboard),
            ..Default::default()
        };

        ctx.send_message(&message.channel_id, &params).await?;
        Ok(())
    }
}
```

### Poll with Multiple Options

```rust
impl InteractiveBot {
    async fn send_poll_message(&self, ctx: &Context, message: &Message) -> Result<(), BotError> {
        let keyboard = Keyboard {
            content: Some(KeyboardContent {
                rows: vec![
                    KeyboardRow {
                        buttons: vec![
                            KeyboardButton {
                                id: Some("poll_option_1".to_string()),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "Option A 🅰️".to_string(),
                                    visited_label: "Voted for A ✅".to_string(),
                                    style: Some(1),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: Some(1), // One vote per user
                                    data: Some("vote_a".to_string()),
                                    reply: None,
                                    enter: Some(true),
                                }),
                            },
                            KeyboardButton {
                                id: Some("poll_option_2".to_string()),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "Option B 🅱️".to_string(),
                                    visited_label: "Voted for B ✅".to_string(),
                                    style: Some(2),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: Some(1),
                                    data: Some("vote_b".to_string()),
                                    reply: None,
                                    enter: Some(true),
                                }),
                            },
                        ],
                    },
                    KeyboardRow {
                        buttons: vec![
                            KeyboardButton {
                                id: Some("poll_option_3".to_string()),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "Option C 🅲".to_string(),
                                    visited_label: "Voted for C ✅".to_string(),
                                    style: Some(3),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: Some(1),
                                    data: Some("vote_c".to_string()),
                                    reply: None,
                                    enter: Some(true),
                                }),
                            },
                            KeyboardButton {
                                id: Some("poll_results".to_string()),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "View Results 📊".to_string(),
                                    visited_label: "Results Shown".to_string(),
                                    style: Some(4),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: None,
                                    data: Some("show_results".to_string()),
                                    reply: None,
                                    enter: Some(false),
                                }),
                            },
                        ],
                    },
                ],
            }),
        };

        let params = MessageParams {
            content: Some("📊 **Quick Poll: What's your favorite programming language?**\n\nChoose one option below:".to_string()),
            keyboard: Some(keyboard),
            ..Default::default()
        };

        ctx.send_message(&message.channel_id, &params).await?;
        Ok(())
    }
}
```

### Navigation Menu

```rust
impl InteractiveBot {
    async fn send_menu_message(&self, ctx: &Context, message: &Message) -> Result<(), BotError> {
        let keyboard = Keyboard {
            content: Some(KeyboardContent {
                rows: vec![
                    KeyboardRow {
                        buttons: vec![
                            KeyboardButton {
                                id: Some("menu_help".to_string()),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "📖 Help".to_string(),
                                    visited_label: "Help Viewed".to_string(),
                                    style: Some(1),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: None,
                                    data: Some("show_help".to_string()),
                                    reply: None,
                                    enter: Some(false),
                                }),
                            },
                            KeyboardButton {
                                id: Some("menu_settings".to_string()),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "⚙️ Settings".to_string(),
                                    visited_label: "Settings Opened".to_string(),
                                    style: Some(2),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: None,
                                    data: Some("show_settings".to_string()),
                                    reply: None,
                                    enter: Some(false),
                                }),
                            },
                        ],
                    },
                    KeyboardRow {
                        buttons: vec![
                            KeyboardButton {
                                id: Some("menu_stats".to_string()),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "📈 Statistics".to_string(),
                                    visited_label: "Stats Viewed".to_string(),
                                    style: Some(3),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: None,
                                    data: Some("show_stats".to_string()),
                                    reply: None,
                                    enter: Some(false),
                                }),
                            },
                            KeyboardButton {
                                id: Some("menu_about".to_string()),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "ℹ️ About".to_string(),
                                    visited_label: "About Viewed".to_string(),
                                    style: Some(4),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: None,
                                    data: Some("show_about".to_string()),
                                    reply: None,
                                    enter: Some(false),
                                }),
                            },
                        ],
                    },
                ],
            }),
        };

        let params = MessageParams {
            content: Some("🎛️ **Main Menu**\n\nSelect an option to continue:".to_string()),
            keyboard: Some(keyboard),
            ..Default::default()
        };

        ctx.send_message(&message.channel_id, &params).await?;
        Ok(())
    }
}
```

## Rich Interactive Messages with Embeds

### Embed with Interactive Elements

```rust
use botrs::models::message::{Embed, EmbedField, EmbedFooter};

impl InteractiveBot {
    async fn send_rich_interactive_message(&self, ctx: &Context, message: &Message) -> Result<(), BotError> {
        let embed = Embed {
            title: Some("🎮 Game Selection".to_string()),
            description: Some("Choose a game to play with the bot!".to_string()),
            color: Some(0x7289da), // Discord blurple
            fields: vec![
                EmbedField {
                    name: "🎲 Rock Paper Scissors".to_string(),
                    value: "Classic game of chance".to_string(),
                    inline: Some(true),
                },
                EmbedField {
                    name: "🎯 Trivia".to_string(),
                    value: "Test your knowledge".to_string(),
                    inline: Some(true),
                },
                EmbedField {
                    name: "🎪 Random Fun".to_string(),
                    value: "Surprise me!".to_string(),
                    inline: Some(true),
                },
            ],
            footer: Some(EmbedFooter {
                text: "Click a button below to start".to_string(),
                icon_url: None,
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
            ..Default::default()
        };

        let keyboard = Keyboard {
            content: Some(KeyboardContent {
                rows: vec![
                    KeyboardRow {
                        buttons: vec![
                            KeyboardButton {
                                id: Some("game_rps".to_string()),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "🎲 Rock Paper Scissors".to_string(),
                                    visited_label: "Game Started!".to_string(),
                                    style: Some(1),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: None,
                                    data: Some("start_rps".to_string()),
                                    reply: None,
                                    enter: Some(true),
                                }),
                            },
                        ],
                    },
                    KeyboardRow {
                        buttons: vec![
                            KeyboardButton {
                                id: Some("game_trivia".to_string()),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "🎯 Start Trivia".to_string(),
                                    visited_label: "Trivia Started!".to_string(),
                                    style: Some(2),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: None,
                                    data: Some("start_trivia".to_string()),
                                    reply: None,
                                    enter: Some(true),
                                }),
                            },
                            KeyboardButton {
                                id: Some("game_random".to_string()),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "🎪 Random Fun".to_string(),
                                    visited_label: "Surprise Activated!".to_string(),
                                    style: Some(3),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: None,
                                    data: Some("start_random".to_string()),
                                    reply: None,
                                    enter: Some(true),
                                }),
                            },
                        ],
                    },
                ],
            }),
        };

        let params = MessageParams {
            embed: Some(embed),
            keyboard: Some(keyboard),
            ..Default::default()
        };

        ctx.send_message(&message.channel_id, &params).await?;
        Ok(())
    }
}
```

## Advanced Interactive Patterns

### Multi-Step Interaction

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct InteractionState {
    pub user_id: String,
    pub step: u32,
    pub data: HashMap<String, String>,
}

pub struct AdvancedInteractiveBot {
    interaction_states: Arc<Mutex<HashMap<String, InteractionState>>>,
}

impl AdvancedInteractiveBot {
    pub fn new() -> Self {
        Self {
            interaction_states: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn start_setup_wizard(&self, ctx: &Context, message: &Message) -> Result<(), BotError> {
        let user_id = message.author.as_ref().unwrap().id.clone();
        
        // Initialize interaction state
        {
            let mut states = self.interaction_states.lock().await;
            states.insert(user_id.clone(), InteractionState {
                user_id: user_id.clone(),
                step: 1,
                data: HashMap::new(),
            });
        }

        let keyboard = Keyboard {
            content: Some(KeyboardContent {
                rows: vec![
                    KeyboardRow {
                        buttons: vec![
                            KeyboardButton {
                                id: Some("setup_step1_beginner".to_string()),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "🌱 Beginner".to_string(),
                                    visited_label: "Beginner Selected".to_string(),
                                    style: Some(1),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: None,
                                    data: Some("level_beginner".to_string()),
                                    reply: None,
                                    enter: Some(true),
                                }),
                            },
                            KeyboardButton {
                                id: Some("setup_step1_intermediate".to_string()),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "🌿 Intermediate".to_string(),
                                    visited_label: "Intermediate Selected".to_string(),
                                    style: Some(2),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: None,
                                    data: Some("level_intermediate".to_string()),
                                    reply: None,
                                    enter: Some(true),
                                }),
                            },
                            KeyboardButton {
                                id: Some("setup_step1_advanced".to_string()),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "🌳 Advanced".to_string(),
                                    visited_label: "Advanced Selected".to_string(),
                                    style: Some(3),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: None,
                                    data: Some("level_advanced".to_string()),
                                    reply: None,
                                    enter: Some(true),
                                }),
                            },
                        ],
                    },
                ],
            }),
        };

        let params = MessageParams {
            content: Some("🛠️ **Setup Wizard - Step 1/3**\n\nWhat's your experience level?".to_string()),
            keyboard: Some(keyboard),
            ..Default::default()
        };

        ctx.send_message(&message.channel_id, &params).await?;
        Ok(())
    }

    async fn handle_setup_step2(&self, ctx: &Context, channel_id: &str, user_id: &str) -> Result<(), BotError> {
        let keyboard = Keyboard {
            content: Some(KeyboardContent {
                rows: vec![
                    KeyboardRow {
                        buttons: vec![
                            KeyboardButton {
                                id: Some("setup_step2_gaming".to_string()),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "🎮 Gaming".to_string(),
                                    visited_label: "Gaming Selected".to_string(),
                                    style: Some(1),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: None,
                                    data: Some("interest_gaming".to_string()),
                                    reply: None,
                                    enter: Some(true),
                                }),
                            },
                            KeyboardButton {
                                id: Some("setup_step2_music".to_string()),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "🎵 Music".to_string(),
                                    visited_label: "Music Selected".to_string(),
                                    style: Some(2),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: None,
                                    data: Some("interest_music".to_string()),
                                    reply: None,
                                    enter: Some(true),
                                }),
                            },
                        ],
                    },
                    KeyboardRow {
                        buttons: vec![
                            KeyboardButton {
                                id: Some("setup_step2_tech".to_string()),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "💻 Technology".to_string(),
                                    visited_label: "Tech Selected".to_string(),
                                    style: Some(3),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: None,
                                    data: Some("interest_tech".to_string()),
                                    reply: None,
                                    enter: Some(true),
                                }),
                            },
                            KeyboardButton {
                                id: Some("setup_step2_art".to_string()),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "🎨 Art & Design".to_string(),
                                    visited_label: "Art Selected".to_string(),
                                    style: Some(4),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: None,
                                    data: Some("interest_art".to_string()),
                                    reply: None,
                                    enter: Some(true),
                                }),
                            },
                        ],
                    },
                ],
            }),
        };

        let params = MessageParams {
            content: Some("🛠️ **Setup Wizard - Step 2/3**\n\nWhat are your main interests?".to_string()),
            keyboard: Some(keyboard),
            ..Default::default()
        };

        ctx.send_message(channel_id, &params).await?;
        Ok(())
    }

    async fn complete_setup(&self, ctx: &Context, channel_id: &str, user_id: &str) -> Result<(), BotError> {
        let state = {
            let states = self.interaction_states.lock().await;
            states.get(user_id).cloned()
        };

        if let Some(user_state) = state {
            let level = user_state.data.get("level").unwrap_or(&"unknown".to_string());
            let interest = user_state.data.get("interest").unwrap_or(&"unknown".to_string());

            let embed = Embed {
                title: Some("✅ Setup Complete!".to_string()),
                description: Some("Your preferences have been saved.".to_string()),
                color: Some(0x00ff00), // Green
                fields: vec![
                    EmbedField {
                        name: "Experience Level".to_string(),
                        value: level.clone(),
                        inline: Some(true),
                    },
                    EmbedField {
                        name: "Primary Interest".to_string(),
                        value: interest.clone(),
                        inline: Some(true),
                    },
                ],
                footer: Some(EmbedFooter {
                    text: "You can change these settings anytime with !setup".to_string(),
                    icon_url: None,
                }),
                ..Default::default()
            };

            let params = MessageParams {
                embed: Some(embed),
                ..Default::default()
            };

            ctx.send_message(channel_id, &params).await?;

            // Clean up interaction state
            let mut states = self.interaction_states.lock().await;
            states.remove(user_id);
        }

        Ok(())
    }
}
```

## Best Practices

### 1. Button State Management

```rust
pub struct ButtonStateManager {
    button_states: Arc<Mutex<HashMap<String, ButtonState>>>,
}

#[derive(Clone)]
pub struct ButtonState {
    pub enabled: bool,
    pub click_count: u32,
    pub last_clicked: Option<chrono::DateTime<chrono::Utc>>,
    pub clicked_by: Vec<String>,
}

impl ButtonStateManager {
    pub async fn handle_button_click(&self, button_id: &str, user_id: &str) -> bool {
        let mut states = self.button_states.lock().await;
        let state = states.entry(button_id.to_string()).or_insert(ButtonState {
            enabled: true,
            click_count: 0,
            last_clicked: None,
            clicked_by: Vec::new(),
        });

        if !state.enabled {
            return false;
        }

        // Check if user already clicked (for polls)
        if state.clicked_by.contains(&user_id.to_string()) {
            return false;
        }

        state.click_count += 1;
        state.last_clicked = Some(chrono::Utc::now());
        state.clicked_by.push(user_id.to_string());

        true
    }
}
```

### 2. Timeout Handling

```rust
impl InteractiveBot {
    async fn send_timed_interactive_message(&self, ctx: &Context, message: &Message) -> Result<(), BotError> {
        let keyboard = self.create_timed_keyboard();
        
        let params = MessageParams {
            content: Some("⏰ **Timed Poll** (expires in 60 seconds)\n\nVote now!".to_string()),
            keyboard: Some(keyboard),
            ..Default::default()
        };

        let sent_message = ctx.send_message(&message.channel_id, &params).await?;

        // Schedule message update after timeout
        let ctx_clone = ctx.clone();
        let channel_id = message.channel_id.clone();
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
            
            let expired_params = MessageParams {
                content: Some("⏰ **Poll Expired**\n\nThis poll has ended. Thanks to everyone who participated!".to_string()),
                ..Default::default()
            };

            // Note: In a real implementation, you'd need message editing capabilities
            // For now, we send a new message
            let _ = ctx_clone.send_message(&channel_id, &expired_params).await;
        });

        Ok(())
    }

    fn create_timed_keyboard(&self) -> Keyboard {
        // Create keyboard with timestamp in button data
        let timestamp = chrono::Utc::now().timestamp();
        
        Keyboard {
            content: Some(KeyboardContent {
                rows: vec![
                    KeyboardRow {
                        buttons: vec![
                            KeyboardButton {
                                id: Some(format!("timed_yes_{}", timestamp)),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "👍 Yes".to_string(),
                                    visited_label: "Voted Yes".to_string(),
                                    style: Some(1),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: Some(1),
                                    data: Some(format!("vote_yes_{}", timestamp)),
                                    reply: None,
                                    enter: Some(true),
                                }),
                            },
                            KeyboardButton {
                                id: Some(format!("timed_no_{}", timestamp)),
                                render_data: Some(KeyboardButtonRenderData {
                                    label: "👎 No".to_string(),
                                    visited_label: "Voted No".to_string(),
                                    style: Some(2),
                                }),
                                action: Some(KeyboardButtonAction {
                                    action_type: Some(2),
                                    permission: None,
                                    click_limit: Some(1),
                                    data: Some(format!("vote_no_{}", timestamp)),
                                    reply: None,
                                    enter: Some(true),
                                }),
                            },
                        ],
                    },
                ],
            }),
        }
    }
}
```

## Usage Examples

### Basic Interactive Commands

```
# Send a simple button
!button

# Create a poll
!poll

# Show navigation menu
!menu

# Start setup wizard
!setup
```

### Advanced Features

- **Multi-step interactions**: Guide users through complex workflows
- **State persistence**: Remember user choices across sessions
- **Conditional buttons**: Show different options based on user state
- **Timed interactions**: Auto-expire interactive elements
- **Permission-based buttons**: Show buttons only to authorized users

## Integration Tips

1. **Combine with embeds**: Use rich embeds to provide context for interactive elements
2. **Handle timeouts**: Always have fallback behavior for expired interactions
3. **Validate permissions**: Check user permissions before showing sensitive buttons
4. **Provide feedback**: Always acknowledge button clicks with appropriate responses
5. **Clean up state**: Remove interaction states after completion to prevent memory leaks

## See Also

- [Rich Messages](./rich-messages.md) - Advanced message formatting
- [Command Handler](./command-handler.md) - Structured command processing
- [Event Handling](./event-handling.md) - Comprehensive event processing
- [File Uploads](./file-uploads.md) - Working with attachments and media