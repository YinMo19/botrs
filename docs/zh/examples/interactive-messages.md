# 交互式消息示例

本示例展示如何在 BotRS 机器人中创建和处理交互式消息，包括按钮、选择菜单、表单等交互组件。

## 概述

QQ 频道支持多种交互式消息组件，允许用户通过点击按钮、选择选项等方式与机器人进行交互，而不仅仅是发送文本消息。这些交互组件包括：

- **内联按钮**: 消息下方的可点击按钮
- **选择菜单**: 下拉选择列表
- **键盘布局**: 自定义键盘布局
- **模态表单**: 弹出式表单输入

## 基础按钮消息

### 简单按钮

```rust
use botrs::{Context, EventHandler, Message, MessageParams, MessageKeyboard, KeyboardButton, KeyboardRow};

async fn send_simple_button(
    ctx: &Context,
    channel_id: &str
) -> Result<(), botrs::BotError> {
    let keyboard = MessageKeyboard::new()
        .add_row(KeyboardRow::new()
            .add_button(KeyboardButton::new("点击我", "button_clicked"))
        );

    let params = MessageParams::new_text("这是一个带按钮的消息")
        .with_keyboard(keyboard);

    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    Ok(())
}
```

### 多按钮布局

```rust
async fn send_multi_button_message(
    ctx: &Context,
    channel_id: &str
) -> Result<(), botrs::BotError> {
    let keyboard = MessageKeyboard::new()
        // 第一行：操作按钮
        .add_row(KeyboardRow::new()
            .add_button(KeyboardButton::new("✅ 同意", "action_agree"))
            .add_button(KeyboardButton::new("❌ 拒绝", "action_reject"))
        )
        // 第二行：信息按钮
        .add_row(KeyboardRow::new()
            .add_button(KeyboardButton::new("ℹ️ 详情", "show_details"))
            .add_button(KeyboardButton::new("❓ 帮助", "show_help"))
        )
        // 第三行：导航按钮
        .add_row(KeyboardRow::new()
            .add_button(KeyboardButton::new("⬅️ 上一页", "page_prev"))
            .add_button(KeyboardButton::new("🏠 主页", "page_home"))
            .add_button(KeyboardButton::new("➡️ 下一页", "page_next"))
        );

    let params = MessageParams::new_text("请选择您的操作:")
        .with_keyboard(keyboard);

    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    Ok(())
}
```

### 链接按钮

```rust
async fn send_link_buttons(
    ctx: &Context,
    channel_id: &str
) -> Result<(), botrs::BotError> {
    let keyboard = MessageKeyboard::new()
        .add_row(KeyboardRow::new()
            .add_button(
                KeyboardButton::new("🌐 访问官网", "visit_website")
                    .with_url("https://example.com")
            )
            .add_button(
                KeyboardButton::new("📚 查看文档", "view_docs")
                    .with_url("https://docs.example.com")
            )
        )
        .add_row(KeyboardRow::new()
            .add_button(
                KeyboardButton::new("💬 加入群聊", "join_group")
                    .with_url("https://qun.qq.com/qqweb/qunpro/share?_wv=3&_wwv=128&inviteCode=example")
            )
        );

    let params = MessageParams::new_text("相关链接:")
        .with_keyboard(keyboard);

    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    Ok(())
}
```

## 动态交互界面

### 分页界面

```rust
#[derive(Clone)]
pub struct PaginatedData {
    pub items: Vec<String>,
    pub current_page: usize,
    pub items_per_page: usize,
}

impl PaginatedData {
    pub fn new(items: Vec<String>, items_per_page: usize) -> Self {
        Self {
            items,
            current_page: 0,
            items_per_page,
        }
    }

    pub fn total_pages(&self) -> usize {
        (self.items.len() + self.items_per_page - 1) / self.items_per_page
    }

    pub fn current_items(&self) -> &[String] {
        let start = self.current_page * self.items_per_page;
        let end = std::cmp::min(start + self.items_per_page, self.items.len());
        &self.items[start..end]
    }

    pub fn has_prev(&self) -> bool {
        self.current_page > 0
    }

    pub fn has_next(&self) -> bool {
        self.current_page < self.total_pages() - 1
    }

    pub fn prev_page(&mut self) {
        if self.has_prev() {
            self.current_page -= 1;
        }
    }

    pub fn next_page(&mut self) {
        if self.has_next() {
            self.current_page += 1;
        }
    }
}

async fn send_paginated_list(
    ctx: &Context,
    channel_id: &str,
    data: &PaginatedData
) -> Result<(), botrs::BotError> {
    let mut content = format!("📄 第 {} 页 / 共 {} 页\n\n", data.current_page + 1, data.total_pages());

    for (index, item) in data.current_items().iter().enumerate() {
        content.push_str(&format!("{}. {}\n", data.current_page * data.items_per_page + index + 1, item));
    }

    let mut keyboard = MessageKeyboard::new();

    // 导航按钮行
    let mut nav_row = KeyboardRow::new();

    if data.has_prev() {
        nav_row = nav_row.add_button(KeyboardButton::new("⬅️ 上一页", "page_prev"));
    }

    nav_row = nav_row.add_button(KeyboardButton::new("🔄 刷新", "page_refresh"));

    if data.has_next() {
        nav_row = nav_row.add_button(KeyboardButton::new("➡️ 下一页", "page_next"));
    }

    keyboard = keyboard.add_row(nav_row);

    // 操作按钮行
    keyboard = keyboard.add_row(KeyboardRow::new()
        .add_button(KeyboardButton::new("➕ 添加项目", "add_item"))
        .add_button(KeyboardButton::new("🗑️ 删除模式", "delete_mode"))
    );

    let params = MessageParams::new_text(&content)
        .with_keyboard(keyboard);

    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    Ok(())
}
```

### 菜单选择界面

```rust
pub struct MenuOption {
    pub id: String,
    pub label: String,
    pub description: String,
    pub emoji: String,
}

async fn send_menu_selection(
    ctx: &Context,
    channel_id: &str,
    title: &str,
    options: &[MenuOption]
) -> Result<(), botrs::BotError> {
    let mut content = format!("📋 {}\n\n", title);
    content.push_str("请选择一个选项:\n\n");

    for option in options {
        content.push_str(&format!("{} **{}**\n{}\n\n", option.emoji, option.label, option.description));
    }

    let mut keyboard = MessageKeyboard::new();
    let mut current_row = KeyboardRow::new();

    for (index, option) in options.iter().enumerate() {
        current_row = current_row.add_button(
            KeyboardButton::new(
                &format!("{} {}", option.emoji, option.label),
                &format!("menu_select_{}", option.id)
            )
        );

        // 每行最多3个按钮
        if (index + 1) % 3 == 0 || index == options.len() - 1 {
            keyboard = keyboard.add_row(current_row);
            current_row = KeyboardRow::new();
        }
    }

    // 添加取消按钮
    keyboard = keyboard.add_row(KeyboardRow::new()
        .add_button(KeyboardButton::new("❌ 取消", "menu_cancel"))
    );

    let params = MessageParams::new_text(&content)
        .with_keyboard(keyboard);

    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    Ok(())
}
```

## 表单和输入收集

### 简单表单界面

```rust
#[derive(Clone)]
pub struct FormData {
    pub form_id: String,
    pub fields: std::collections::HashMap<String, String>,
    pub current_field: Option<String>,
    pub completed: bool,
}

impl FormData {
    pub fn new(form_id: String) -> Self {
        Self {
            form_id,
            fields: std::collections::HashMap::new(),
            current_field: None,
            completed: false,
        }
    }
}

async fn send_form_interface(
    ctx: &Context,
    channel_id: &str,
    form_data: &FormData
) -> Result<(), botrs::BotError> {
    let content = if form_data.completed {
        format!("✅ 表单已完成!\n\n📋 表单内容:\n{}", format_form_summary(form_data))
    } else {
        format!("📝 请填写表单信息\n\n{}", format_form_progress(form_data))
    };

    let keyboard = if form_data.completed {
        MessageKeyboard::new()
            .add_row(KeyboardRow::new()
                .add_button(KeyboardButton::new("📤 提交", "form_submit"))
                .add_button(KeyboardButton::new("✏️ 修改", "form_edit"))
                .add_button(KeyboardButton::new("❌ 取消", "form_cancel"))
            )
    } else {
        create_form_keyboard(form_data)
    };

    let params = MessageParams::new_text(&content)
        .with_keyboard(keyboard);

    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    Ok(())
}

fn format_form_progress(form_data: &FormData) -> String {
    let mut progress = String::new();

    let fields = vec![
        ("name", "姓名"),
        ("email", "邮箱"),
        ("phone", "电话"),
        ("message", "留言"),
    ];

    for (field_id, field_name) in fields {
        if let Some(value) = form_data.fields.get(field_id) {
            progress.push_str(&format!("✅ {}: {}\n", field_name, value));
        } else {
            progress.push_str(&format!("⭕ {}: 待填写\n", field_name));
        }
    }

    progress
}

fn format_form_summary(form_data: &FormData) -> String {
    let mut summary = String::new();

    for (key, value) in &form_data.fields {
        summary.push_str(&format!("• {}: {}\n", key, value));
    }

    summary
}

fn create_form_keyboard(form_data: &FormData) -> MessageKeyboard {
    let mut keyboard = MessageKeyboard::new();

    // 字段填写按钮
    keyboard = keyboard.add_row(KeyboardRow::new()
        .add_button(KeyboardButton::new(
            if form_data.fields.contains_key("name") { "✅ 姓名" } else { "⭕ 姓名" },
            "form_field_name"
        ))
        .add_button(KeyboardButton::new(
            if form_data.fields.contains_key("email") { "✅ 邮箱" } else { "⭕ 邮箱" },
            "form_field_email"
        ))
    );

    keyboard = keyboard.add_row(KeyboardRow::new()
        .add_button(KeyboardButton::new(
            if form_data.fields.contains_key("phone") { "✅ 电话" } else { "⭕ 电话" },
            "form_field_phone"
        ))
        .add_button(KeyboardButton::new(
            if form_data.fields.contains_key("message") { "✅ 留言" } else { "⭕ 留言" },
            "form_field_message"
        ))
    );

    // 控制按钮
    let all_filled = vec!["name", "email", "phone", "message"]
        .iter()
        .all(|field| form_data.fields.contains_key(*field));

    if all_filled {
        keyboard = keyboard.add_row(KeyboardRow::new()
            .add_button(KeyboardButton::new("✅ 完成填写", "form_complete"))
        );
    }

    keyboard = keyboard.add_row(KeyboardRow::new()
        .add_button(KeyboardButton::new("🗑️ 清空", "form_clear"))
        .add_button(KeyboardButton::new("❌ 取消", "form_cancel"))
    );

    keyboard
}
```

## 游戏和投票界面

### 投票系统

```rust
#[derive(Clone)]
pub struct PollData {
    pub poll_id: String,
    pub question: String,
    pub options: Vec<String>,
    pub votes: std::collections::HashMap<String, usize>, // option_index -> vote_count
    pub voters: std::collections::HashMap<String, usize>, // user_id -> option_index
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl PollData {
    pub fn new(poll_id: String, question: String, options: Vec<String>) -> Self {
        Self {
            poll_id,
            question,
            options,
            votes: std::collections::HashMap::new(),
            voters: std::collections::HashMap::new(),
            is_active: true,
            created_at: chrono::Utc::now(),
            expires_at: None,
        }
    }

    pub fn vote(&mut self, user_id: &str, option_index: usize) -> Result<(), String> {
        if !self.is_active {
            return Err("投票已结束".to_string());
        }

        if option_index >= self.options.len() {
            return Err("无效的选项".to_string());
        }

        // 如果用户之前已投票，先取消之前的投票
        if let Some(&old_option) = self.voters.get(user_id) {
            let old_count = self.votes.get(&old_option.to_string()).unwrap_or(&0);
            if *old_count > 0 {
                self.votes.insert(old_option.to_string(), old_count - 1);
            }
        }

        // 记录新投票
        self.voters.insert(user_id.to_string(), option_index);
        let new_count = self.votes.get(&option_index.to_string()).unwrap_or(&0) + 1;
        self.votes.insert(option_index.to_string(), new_count);

        Ok(())
    }

    pub fn get_results(&self) -> Vec<(String, usize, f64)> {
        let total_votes: usize = self.votes.values().sum();

        self.options.iter().enumerate().map(|(i, option)| {
            let vote_count = *self.votes.get(&i.to_string()).unwrap_or(&0);
            let percentage = if total_votes > 0 {
                (vote_count as f64 / total_votes as f64) * 100.0
            } else {
                0.0
            };
            (option.clone(), vote_count, percentage)
        }).collect()
    }
}

async fn send_poll_message(
    ctx: &Context,
    channel_id: &str,
    poll: &PollData
) -> Result<(), botrs::BotError> {
    let results = poll.get_results();
    let total_votes: usize = results.iter().map(|(_, count, _)| count).sum();

    let mut content = format!("📊 **{}**\n\n", poll.question);

    if total_votes > 0 {
        content.push_str("当前结果:\n");
        for (option, count, percentage) in &results {
            let bar = create_progress_bar(percentage / 100.0, 10);
            content.push_str(&format!("**{}** {} ({:.1}%) - {} 票\n", option, bar, percentage, count));
        }
        content.push_str(&format!("\n📈 总投票数: {}\n", total_votes));
    } else {
        content.push_str("还没有人投票，快来投出第一票吧!\n\n");
        for (i, option) in poll.options.iter().enumerate() {
            content.push_str(&format!("{}. {}\n", i + 1, option));
        }
    }

    if let Some(expires_at) = poll.expires_at {
        let remaining = expires_at - chrono::Utc::now();
        if remaining.num_seconds() > 0 {
            content.push_str(&format!("\n⏰ 剩余时间: {} 分钟", remaining.num_minutes()));
        }
    }

    let mut keyboard = MessageKeyboard::new();

    // 投票选项按钮
    let mut option_rows = Vec::new();
    let mut current_row = KeyboardRow::new();

    for (i, option) in poll.options.iter().enumerate() {
        let emoji = match i {
            0 => "🅰️",
            1 => "🅱️",
            2 => "🅲️",
            3 => "🅳️",
            4 => "🅴️",
            _ => "▫️",
        };

        current_row = current_row.add_button(
            KeyboardButton::new(
                &format!("{} {}", emoji, option),
                &format!("poll_vote_{}_{}", poll.poll_id, i)
            )
        );

        // 每行最多2个选项
        if (i + 1) % 2 == 0 || i == poll.options.len() - 1 {
            option_rows.push(current_row);
            current_row = KeyboardRow::new();
        }
    }

    for row in option_rows {
        keyboard = keyboard.add_row(row);
    }

    // 控制按钮
    if poll.is_active {
        keyboard = keyboard.add_row(KeyboardRow::new()
            .add_button(KeyboardButton::new("🔄 刷新结果", &format!("poll_refresh_{}", poll.poll_id)))
            .add_button(KeyboardButton::new("⏹️ 结束投票", &format!("poll_end_{}", poll.poll_id)))
        );
    } else {
        keyboard = keyboard.add_row(KeyboardRow::new()
            .add_button(KeyboardButton::new("📊 最终结果", &format!("poll_final_{}", poll.poll_id)))
        );
    }

    let params = MessageParams::new_text(&content)
        .with_keyboard(keyboard);

    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    Ok(())
}

fn create_progress_bar(percentage: f64, length: usize) -> String {
    let filled = (percentage * length as f64) as usize;
    let empty = length - filled;
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}
```

### 猜数字游戏

```rust
#[derive(Clone)]
pub struct GuessGameData {
    pub game_id: String,
    pub target_number: u32,
    pub min_range: u32,
    pub max_range: u32,
    pub attempts: Vec<u32>,
    pub max_attempts: u32,
    pub is_active: bool,
    pub winner: Option<String>,
}

impl GuessGameData {
    pub fn new(game_id: String, min_range: u32, max_range: u32, max_attempts: u32) -> Self {
        use rand::Rng;
        let target_number = rand::thread_rng().gen_range(min_range..=max_range);

        Self {
            game_id,
            target_number,
            min_range,
            max_range,
            attempts: Vec::new(),
            max_attempts,
            is_active: true,
            winner: None,
        }
    }

    pub fn make_guess(&mut self, user_id: &str, guess: u32) -> GuessResult {
        if !self.is_active {
            return GuessResult::GameEnded;
        }

        self.attempts.push(guess);

        if guess == self.target_number {
            self.is_active = false;
            self.winner = Some(user_id.to_string());
            GuessResult::Correct
        } else if self.attempts.len() >= self.max_attempts as usize {
            self.is_active = false;
            GuessResult::GameOver
        } else if guess < self.target_number {
            GuessResult::TooLow
        } else {
            GuessResult::TooHigh
        }
    }
}

#[derive(Debug)]
pub enum GuessResult {
    TooLow,
    TooHigh,
    Correct,
    GameOver,
    GameEnded,
}

async fn send_guess_game(
    ctx: &Context,
    channel_id: &str,
    game: &GuessGameData
) -> Result<(), botrs::BotError> {
    let mut content = format!("🎯 **猜数字游戏** (游戏 ID: {})\n\n", game.game_id);
    content.push_str(&format!("🎲 范围: {} - {}\n", game.min_range, game.max_range));
    content.push_str(&format!("🎪 最大尝试次数: {}\n", game.max_attempts));
    content.push_str(&format!("📊 已尝试: {}/{}\n\n", game.attempts.len(), game.max_attempts));

    if !game.attempts.is_empty() {
        content.push_str("🔍 历史猜测: ");
        for (i, attempt) in game.attempts.iter().enumerate() {
            if i > 0 { content.push_str(", "); }
            content.push_str(&attempt.to_string());
        }
        content.push_str("\n\n");
    }

    if let Some(ref winner) = game.winner {
        content.push_str(&format!("🎉 恭喜 {} 猜中了数字 {}!", winner, game.target_number));
    } else if !game.is_active {
        content.push_str(&format!("💔 游戏结束! 正确答案是: {}", game.target_number));
    } else {
        content.push_str("🤔 请选择一个数字或输入自定义数字:");
    }

    let mut keyboard = MessageKeyboard::new();

    if game.is_active {
        // 快速选择按钮（基于当前范围）
        let mut current_min = game.min_range;
        let mut current_max = game.max_range;

        // 根据之前的猜测调整范围提示
        if let Some(&last_guess) = game.attempts.last() {
            if last_guess < game.target_number {
                current_min = last_guess + 1;
            } else if last_guess > game.target_number {
                current_max = last_guess - 1;
            }
        }

        // 生成一些建议数字
        let suggestions = generate_guess_suggestions(current_min, current_max, 6);

        let mut suggestion_rows = Vec::new();
        let mut current_row = KeyboardRow::new();

        for (i, suggestion) in suggestions.iter().enumerate() {
            current_row = current_row.add_button(
                KeyboardButton::new(
                    &suggestion.to_string(),
                    &format!("game_guess_{}_{}", game.game_id, suggestion)
                )
            );

            if (i + 1) % 3 == 0 || i == suggestions.len() - 1 {
                suggestion_rows.push(current_row);
                current_row = KeyboardRow::new();
            }
        }

        for row in suggestion_rows {
            keyboard = keyboard.add_row(row);
        }

        // 控制按钮
        keyboard = keyboard.add_row(KeyboardRow::new()
            .add_button(KeyboardButton::new("🎲 随机猜测", &format!("game_random_{}", game.game_id)))
            .add_button(KeyboardButton::new("💡 提示", &format!("game_hint_{}", game.game_id)))
        );

        keyboard = keyboard.add_row(KeyboardRow::new()
            .add_button(KeyboardButton::new("❌ 放弃游戏", &format!("game_quit_{}", game.game_id)))
        );
    } else {
        // 游戏结束后的选项
        keyboard = keyboard.add_row(KeyboardRow::new()
            .add_button(KeyboardButton::new("🔄 再来一局", "game_new"))
            .add_button(KeyboardButton::new("📊 查看统计", "game_stats"))
        );
    }

    let params = MessageParams::new_text(&content)
        .with_keyboard(keyboard);

    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    Ok(())
}

fn generate_guess_suggestions(min: u32, max: u32, count: usize) -> Vec<u32> {
    use rand::seq::SliceRandom;

    if max <= min {
        return vec![];
    }

    let mut suggestions = Vec::new();
    let range = max - min + 1;

    if range <= count as u32 {
        // 如果范围很小，就列出所有可能的数字
        suggestions.extend(min..=max);
    } else {
        // 生成一些有策略的建议
        let mid = (min + max) / 2;
        suggestions.push(mid);

        // 添加一些随机数字
        let mut rng = rand::thread_rng();
        let all_numbers: Vec<u32> = (min..=max).collect();
        let mut random_numbers = all_numbers.choose_multiple(&mut rng, count - 1).cloned().collect::<Vec<_>>();
        random_numbers.sort();
        suggestions.extend(random_numbers);
    }

    suggestions.sort();
    suggestions.dedup();
    suggestions.truncate(count);
    suggestions
}
```

### 高级功能
- **多步骤交互**：引导用户完成复杂的操作流程
- **状态持久化**：记住用户在不同会话中的选择
- **条件按钮**：根据用户状态显示不同的选项
- **定时交互**：自动使交互元素过期
- **基于权限的按钮**：仅向授权用户显示按钮

## 集成提示
1. **结合嵌入内容**：使用丰富的嵌入内容为互动元素提供上下文信息
2. **处理超时**：对于过期的交互，始终要有备用行为
3. **验证权限**：在显示敏感按钮之前检查用户权限
4. **提供反馈**：始终以恰当的回应确认按钮点击操作
5. **清理状态**：完成操作后移除交互状态，以防止内存泄漏
另请参阅
- [富文本消息](./rich-messages.md) - 高级消息格式化
- [命令处理程序](./command-handler.md) - 结构化命令处理
- [事件处理](./event-handling.md) - 全面的事件处理
- [文件上传](./file-uploads.md) - 处理附件和媒体文件
