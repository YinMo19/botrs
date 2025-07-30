# BotRS Documentation

This directory contains the comprehensive documentation for BotRS, a Rust QQ Bot framework.

## Documentation Structure

```
docs/
├── .vitepress/          # VitePress configuration
│   └── config.mts       # Site configuration
├── guide/               # English guides
│   ├── introduction.md
│   ├── installation.md
│   ├── quick-start.md
│   └── configuration.md
├── api/                 # English API reference
│   ├── client.md
│   └── event-handler.md
├── examples/            # English examples
│   └── getting-started.md
├── zh/                  # Chinese documentation
│   ├── guide/           # Chinese guides
│   ├── api/             # Chinese API reference
│   └── examples/        # Chinese examples
├── changelog.md         # Version history
├── contributing.md      # Contribution guidelines
└── index.md            # Homepage
```

## Languages Supported

- **English**: Primary documentation in `/guide`, `/api`, `/examples`
- **Chinese (简体中文)**: Complete translation in `/zh` directory

## Building Documentation

### Prerequisites

- Node.js 16+ 
- npm 7+

### Local Development

```bash
# Install dependencies
npm install

# Start development server
npm run docs:dev

# Build for production
npm run docs:build

# Preview production build
npm run docs:preview
```

### Available Scripts

- `npm run docs:dev` - Start development server with hot reload
- `npm run docs:build` - Build static site for production
- `npm run docs:preview` - Preview production build locally
- `npm run docs:serve` - Serve built documentation

## Documentation Guidelines

### Writing Guidelines

1. **Clarity**: Write clear, concise explanations
2. **Examples**: Include practical code examples
3. **Consistency**: Use consistent terminology throughout
4. **Accessibility**: Ensure content is accessible to beginners

### Code Examples

- Use working, tested code examples
- Include imports and necessary setup
- Provide context for each example
- Follow Rust best practices

### Bilingual Content

When adding new content:

1. Write in English first
2. Add corresponding Chinese translation
3. Ensure both versions cover the same topics
4. Update navigation in both languages

### File Naming

- Use kebab-case for file names: `quick-start.md`
- Keep names descriptive but concise
- Maintain parallel structure between languages

## Content Organization

### Guide Section
- **Introduction**: Overview and concepts
- **Installation**: Setup instructions
- **Quick Start**: Get users up and running quickly
- **Configuration**: Detailed configuration options
- **Advanced Topics**: Complex features and patterns

### API Reference
- **Core Components**: Client, EventHandler, Context
- **Models**: Data structures and types
- **Utilities**: Helper functions and types
- **Error Handling**: Error types and patterns

### Examples
- **Getting Started**: Basic examples
- **Common Patterns**: Frequently used code patterns
- **Advanced Use Cases**: Complex real-world examples
- **Migration Guides**: Version upgrade instructions

## Contributing to Documentation

### Adding New Content

1. Create the English version first
2. Add corresponding Chinese translation
3. Update navigation in `.vitepress/config.mts`
4. Test locally before submitting PR

### Updating Existing Content

1. Update both English and Chinese versions
2. Maintain consistency across languages
3. Update any cross-references
4. Verify all links work correctly

### Style Guidelines

- Use present tense for instructions
- Use active voice when possible
- Keep sentences concise and clear
- Use code blocks for all code examples
- Include proper syntax highlighting

### Markdown Features

VitePress supports extended markdown features:

- Syntax highlighting with line numbers
- Custom containers (info, tip, warning, danger)
- Code group tabs
- Math expressions
- Mermaid diagrams

## Deployment

The documentation is automatically deployed when changes are pushed to the main branch. The build process:

1. Installs dependencies
2. Runs `npm run docs:build`
3. Deploys to the hosting platform

## Maintenance

### Regular Tasks

- Keep dependency versions updated
- Review and update outdated content
- Check for broken links
- Verify code examples still work
- Update version references

### Version Updates

When releasing new versions:

1. Update changelog
2. Update version references in code examples
3. Add migration guides if needed
4. Update any deprecated feature warnings

## Getting Help

- **Issues**: Report documentation bugs on GitHub
- **Discussions**: Ask questions in GitHub Discussions
- **Discord**: Join our community Discord server
- **Maintainers**: Contact maintainers for urgent issues

## License

This documentation is licensed under the MIT License, same as the BotRS project.