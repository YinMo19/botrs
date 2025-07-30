import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
    title: "BotRS",
    description: "A Rust QQ Bot framework based on QQ Guild Bot API",
    head: [
        ["link", { rel: "icon", href: "/favicon.ico" }],
        ["meta", { name: "theme-color", content: "#646cff" }],
        ["meta", { name: "og:type", content: "website" }],
        ["meta", { name: "og:locale", content: "en" }],
        ["meta", { name: "og:site_name", content: "BotRS" }],
    ],

    locales: {
        root: {
            label: "English",
            lang: "en",
            title: "BotRS",
            description: "A Rust QQ Bot framework based on QQ Guild Bot API",
            themeConfig: {
                nav: [
                    { text: "Guide", link: "/guide/introduction" },
                    { text: "API Reference", link: "/api/client" },
                    { text: "Examples", link: "/examples/getting-started" },
                    {
                        text: "v0.2.5",
                        items: [
                            { text: "Changelog", link: "/changelog" },
                            { text: "Contributing", link: "/contributing" },
                        ],
                    },
                ],

                sidebar: {
                    "/guide/": [
                        {
                            text: "Getting Started",
                            items: [
                                {
                                    text: "Introduction",
                                    link: "/guide/introduction",
                                },
                                {
                                    text: "Installation",
                                    link: "/guide/installation",
                                },
                                {
                                    text: "Quick Start",
                                    link: "/guide/quick-start",
                                },
                                {
                                    text: "Configuration",
                                    link: "/guide/configuration",
                                },
                            ],
                        },
                        {
                            text: "Core Concepts",
                            items: [
                                {
                                    text: "Client & Event Handler",
                                    link: "/guide/client-handler",
                                },
                                {
                                    text: "Messages & Responses",
                                    link: "/guide/messages",
                                },
                                {
                                    text: "Intents System",
                                    link: "/guide/intents",
                                },
                                {
                                    text: "Error Handling",
                                    link: "/guide/error-handling",
                                },
                            ],
                        },
                        {
                            text: "Advanced Features",
                            items: [
                                {
                                    text: "API Client Usage",
                                    link: "/guide/api-client",
                                },
                                {
                                    text: "WebSocket Gateway",
                                    link: "/guide/gateway",
                                },
                                {
                                    text: "Audio & Media",
                                    link: "/guide/audio-media",
                                },
                                {
                                    text: "Forum & Threads",
                                    link: "/guide/forum-threads",
                                },
                            ],
                        },
                        {
                            text: "Migration Guide",
                            items: [
                                {
                                    text: "v0.2.0 Message API",
                                    link: "/guide/migration-v0.2.0",
                                },
                                {
                                    text: "From Python botpy",
                                    link: "/guide/migration-from-python",
                                },
                            ],
                        },
                    ],
                    "/api/": [
                        {
                            text: "Core API",
                            items: [
                                { text: "Client", link: "/api/client" },
                                {
                                    text: "Event Handler",
                                    link: "/api/event-handler",
                                },
                                { text: "Bot API", link: "/api/bot-api" },
                                { text: "Context", link: "/api/context" },
                            ],
                        },
                        {
                            text: "Models",
                            items: [
                                {
                                    text: "Messages",
                                    link: "/api/models/messages",
                                },
                                {
                                    text: "Guilds & Channels",
                                    link: "/api/models/guilds-channels",
                                },
                                {
                                    text: "Users & Members",
                                    link: "/api/models/users-members",
                                },
                                {
                                    text: "Other Types",
                                    link: "/api/models/other-types",
                                },
                            ],
                        },
                        {
                            text: "Utilities",
                            items: [
                                { text: "Intents", link: "/api/intents" },
                                { text: "Token", link: "/api/token" },
                                {
                                    text: "Error Types",
                                    link: "/api/error-types",
                                },
                            ],
                        },
                    ],
                    "/examples/": [
                        {
                            text: "Basic Examples",
                            items: [
                                {
                                    text: "Getting Started",
                                    link: "/examples/getting-started",
                                },
                                {
                                    text: "Simple Echo Bot",
                                    link: "/examples/echo-bot",
                                },
                                {
                                    text: "Command Handler",
                                    link: "/examples/command-handler",
                                },
                            ],
                        },
                        {
                            text: "Message Types",
                            items: [
                                {
                                    text: "Text Messages",
                                    link: "/examples/text-messages",
                                },
                                {
                                    text: "Rich Messages",
                                    link: "/examples/rich-messages",
                                },
                                {
                                    text: "File Uploads",
                                    link: "/examples/file-uploads",
                                },
                                {
                                    text: "Interactive Messages",
                                    link: "/examples/interactive-messages",
                                },
                            ],
                        },
                        {
                            text: "Advanced Examples",
                            items: [
                                {
                                    text: "Event Handling",
                                    link: "/examples/event-handling",
                                },
                                {
                                    text: "API Integration",
                                    link: "/examples/api-integration",
                                },
                                {
                                    text: "Error Recovery",
                                    link: "/examples/error-recovery",
                                },
                            ],
                        },
                    ],
                },

                socialLinks: [
                    {
                        icon: "github",
                        link: "https://github.com/YinMo19/botrs",
                    },
                ],

                footer: {
                    message: "Released under the MIT License.",
                    copyright: "Copyright © 2024 YinMo19",
                },

                editLink: {
                    pattern:
                        "https://github.com/YinMo19/botrs/edit/main/docs/:path",
                    text: "Edit this page on GitHub",
                },

                search: {
                    provider: "local",
                },
            },
        },
        zh: {
            label: "简体中文",
            lang: "zh-CN",
            title: "BotRS",
            description: "基于 QQ 频道机器人 API 的 Rust QQ 机器人框架",
            themeConfig: {
                nav: [
                    { text: "指南", link: "/zh/guide/introduction" },
                    { text: "API 参考", link: "/zh/api/client" },
                    { text: "示例", link: "/zh/examples/getting-started" },
                    {
                        text: "v0.2.5",
                        items: [
                            { text: "更新日志", link: "/zh/changelog" },
                            { text: "贡献指南", link: "/zh/contributing" },
                        ],
                    },
                ],

                sidebar: {
                    "/zh/guide/": [
                        {
                            text: "开始使用",
                            items: [
                                {
                                    text: "介绍",
                                    link: "/zh/guide/introduction",
                                },
                                {
                                    text: "安装",
                                    link: "/zh/guide/installation",
                                },
                                {
                                    text: "快速开始",
                                    link: "/zh/guide/quick-start",
                                },
                                {
                                    text: "配置",
                                    link: "/zh/guide/configuration",
                                },
                            ],
                        },
                        {
                            text: "核心概念",
                            items: [
                                {
                                    text: "客户端与事件处理",
                                    link: "/zh/guide/client-handler",
                                },
                                {
                                    text: "消息与回复",
                                    link: "/zh/guide/messages",
                                },
                                {
                                    text: "Intent 系统",
                                    link: "/zh/guide/intents",
                                },
                                {
                                    text: "错误处理",
                                    link: "/zh/guide/error-handling",
                                },
                            ],
                        },
                        {
                            text: "高级功能",
                            items: [
                                {
                                    text: "API 客户端使用",
                                    link: "/zh/guide/api-client",
                                },
                                {
                                    text: "WebSocket 网关",
                                    link: "/zh/guide/gateway",
                                },
                                {
                                    text: "音频与媒体",
                                    link: "/zh/guide/audio-media",
                                },
                                {
                                    text: "论坛与话题",
                                    link: "/zh/guide/forum-threads",
                                },
                            ],
                        },
                        {
                            text: "迁移指南",
                            items: [
                                {
                                    text: "v0.2.0 消息 API",
                                    link: "/zh/guide/migration-v0.2.0",
                                },
                                {
                                    text: "从 Python botpy 迁移",
                                    link: "/zh/guide/migration-from-python",
                                },
                            ],
                        },
                    ],
                    "/zh/api/": [
                        {
                            text: "核心 API",
                            items: [
                                { text: "客户端", link: "/zh/api/client" },
                                {
                                    text: "事件处理器",
                                    link: "/zh/api/event-handler",
                                },
                                { text: "机器人 API", link: "/zh/api/bot-api" },
                                { text: "上下文", link: "/zh/api/context" },
                            ],
                        },
                        {
                            text: "数据模型",
                            items: [
                                {
                                    text: "消息",
                                    link: "/zh/api/models/messages",
                                },
                                {
                                    text: "频道与子频道",
                                    link: "/zh/api/models/guilds-channels",
                                },
                                {
                                    text: "用户与成员",
                                    link: "/zh/api/models/users-members",
                                },
                                {
                                    text: "其他类型",
                                    link: "/zh/api/models/other-types",
                                },
                            ],
                        },
                        {
                            text: "工具类",
                            items: [
                                {
                                    text: "Intent 权限",
                                    link: "/zh/api/intents",
                                },
                                { text: "令牌", link: "/zh/api/token" },
                                {
                                    text: "错误类型",
                                    link: "/zh/api/error-types",
                                },
                            ],
                        },
                    ],
                    "/zh/examples/": [
                        {
                            text: "基础示例",
                            items: [
                                {
                                    text: "快速开始",
                                    link: "/zh/examples/getting-started",
                                },
                                {
                                    text: "简单回声机器人",
                                    link: "/zh/examples/echo-bot",
                                },
                                {
                                    text: "命令处理器",
                                    link: "/zh/examples/command-handler",
                                },
                            ],
                        },
                        {
                            text: "消息类型",
                            items: [
                                {
                                    text: "文本消息",
                                    link: "/zh/examples/text-messages",
                                },
                                {
                                    text: "富文本消息",
                                    link: "/zh/examples/rich-messages",
                                },
                                {
                                    text: "文件上传",
                                    link: "/zh/examples/file-uploads",
                                },
                                {
                                    text: "交互式消息",
                                    link: "/zh/examples/interactive-messages",
                                },
                            ],
                        },
                        {
                            text: "高级示例",
                            items: [
                                {
                                    text: "事件处理",
                                    link: "/zh/examples/event-handling",
                                },
                                {
                                    text: "API 集成",
                                    link: "/zh/examples/api-integration",
                                },
                                {
                                    text: "错误恢复",
                                    link: "/zh/examples/error-recovery",
                                },
                            ],
                        },
                    ],
                },

                socialLinks: [
                    {
                        icon: "github",
                        link: "https://github.com/YinMo19/botrs",
                    },
                ],

                footer: {
                    message: "基于 MIT 许可证发布",
                    copyright: "Copyright © 2024 YinMo19",
                },

                editLink: {
                    pattern:
                        "https://github.com/YinMo19/botrs/edit/main/docs/:path",
                    text: "在 GitHub 上编辑此页面",
                },

                search: {
                    provider: "local",
                },

                docFooter: {
                    prev: "上一页",
                    next: "下一页",
                },

                outline: {
                    label: "页面导航",
                },

                lastUpdated: {
                    text: "最后更新于",
                    formatOptions: {
                        dateStyle: "short",
                        timeStyle: "medium",
                    },
                },

                langMenuLabel: "多语言",
                returnToTopLabel: "回到顶部",
                sidebarMenuLabel: "菜单",
                darkModeSwitchLabel: "主题",
                lightModeSwitchTitle: "切换到浅色模式",
                darkModeSwitchTitle: "切换到深色模式",
            },
        },
    },

    themeConfig: {
        logo: "/logo.svg",

        algolia: {
            appId: "BOTRS_APP_ID",
            apiKey: "BOTRS_API_KEY",
            indexName: "botrs",
        },
    },

    markdown: {
        theme: {
            light: "github-light",
            dark: "github-dark",
        },
        lineNumbers: true,
        config: (md) => {
            // Add custom markdown-it plugins here if needed
        },
    },

    // ignoreDeadLinks: true,

    vite: {
        define: {
            __VUE_OPTIONS_API__: false,
        },
    },

    lastUpdated: true,
    cleanUrls: true,

    sitemap: {
        hostname: "https://botrs.yinmo.site",
    },
});
