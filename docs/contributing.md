# Contributing to BotRS

Thank you for your interest in contributing to BotRS! This guide will help you get started with contributing to the project.

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md). Please read it before contributing.

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- A QQ Guild Bot application (for testing)

### Setting Up Your Development Environment

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/botrs.git
   cd botrs
   ```

3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/YinMo19/botrs.git
   ```

4. Install dependencies:
   ```bash
   cargo build
   ```

5. Run tests to ensure everything works:
   ```bash
   cargo test
   ```

## Development Workflow

### Before Making Changes

1. Create a new branch for your feature or fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Keep your branch up to date with upstream:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

### Making Changes

1. Write your code following our [coding standards](#coding-standards)
2. Add tests for new functionality
3. Update documentation if needed
4. Ensure all tests pass:
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

### Submitting Changes

1. Commit your changes with clear, descriptive messages:
   ```bash
   git commit -m "feat: add support for new message type"
   ```

2. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

3. Create a Pull Request on GitHub

## Types of Contributions

### Bug Reports

When reporting bugs, please include:

- A clear description of the issue
- Steps to reproduce the problem
- Expected vs actual behavior
- Your environment (Rust version, OS, etc.)
- Relevant code samples or logs

Use our [bug report template](bug_report.md).

### Feature Requests

For feature requests, please include:

- A clear description of the feature
- The use case and motivation
- Any relevant examples from other libraries
- Willingness to implement the feature yourself

Use our [feature request template](feature_request.md).

### Code Contributions

We welcome contributions for:

- Bug fixes
- New features
- Performance improvements
- Documentation updates
- Example code
- Test coverage improvements

## Coding Standards

### Rust Guidelines

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for code formatting
- Address all `cargo clippy` warnings
- Write comprehensive documentation for public APIs
- Add tests for all new functionality

### Code Style

```rust
// Use descriptive names
pub struct MessageParams {
    content: Option<String>,
    embed: Option<MessageEmbed>,
}

// Document public APIs
/// Creates a new message with text content.
///
/// # Arguments
///
/// * `content` - The text content of the message
///
/// # Example
///
/// ```
/// let params = MessageParams::new_text("Hello, world!");
/// ```
pub fn new_text(content: impl Into<String>) -> Self {
    Self {
        content: Some(content.into()),
        embed: None,
    }
}
```

### Error Handling

- Use `Result<T, BotError>` for fallible operations
- Provide meaningful error messages
- Use `thiserror` for error types
- Include context in error chains

### Testing

- Write unit tests for all public functions
- Use integration tests for complex workflows
- Mock external dependencies when possible
- Aim for high test coverage

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_params_creation() {
        let params = MessageParams::new_text("test");
        assert_eq!(params.content, Some("test".to_string()));
    }

    #[tokio::test]
    async fn test_api_call() {
        // Test async functionality
    }
}
```

## Documentation

### Code Documentation

- Document all public APIs with `///` comments
- Include examples in documentation
- Explain complex algorithms and design decisions
- Use `cargo doc` to generate and review documentation

### User Documentation

- Update relevant guides when adding features
- Include examples in the documentation site
- Keep the changelog updated
- Update README.md if needed

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration_tests
```

### Test Environment

Set up test environment variables:

```bash
export QQ_BOT_APP_ID="test_app_id"
export QQ_BOT_SECRET="test_secret"
export RUST_LOG="debug"
```

### Writing Tests

- Test both success and failure cases
- Use descriptive test names
- Keep tests focused and independent
- Mock external APIs when possible

## Pull Request Process

### Before Submitting

1. Ensure all tests pass
2. Run `cargo clippy` and address warnings
3. Run `cargo fmt` to format code
4. Update documentation if needed
5. Add changelog entry for significant changes

### PR Requirements

- Clear title and description
- Reference related issues
- Include tests for new functionality
- Update documentation
- Follow our commit message format

### Commit Message Format

We use conventional commits:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: new feature
- `fix`: bug fix
- `docs`: documentation changes
- `style`: formatting changes
- `refactor`: code refactoring
- `test`: adding tests
- `chore`: maintenance tasks

Examples:
```
feat(api): add support for voice messages
fix(websocket): resolve connection timeout issue
docs(readme): update installation instructions
```

### Review Process

1. Maintainers will review your PR
2. Address feedback and requested changes
3. Once approved, your PR will be merged
4. Your contribution will be credited in releases

## Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- `MAJOR`: incompatible API changes
- `MINOR`: backwards-compatible functionality
- `PATCH`: backwards-compatible bug fixes

### Release Checklist

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create release tag
4. Publish to crates.io
5. Update documentation

## Community

### Getting Help

- [GitHub Discussions](https://github.com/YinMo19/botrs/discussions) for questions
- [Discord Server](https://discord.gg/botrs) for real-time chat
- [GitHub Issues](https://github.com/YinMo19/botrs/issues) for bugs and features

### Recognition

Contributors are recognized in:

- Release notes
- Contributors section in README
- Hall of Fame in documentation

## License

By contributing to BotRS, you agree that your contributions will be licensed under the MIT License.

## Questions?

If you have questions about contributing, please:

1. Check existing issues and discussions
2. Ask in our Discord server
3. Open a discussion on GitHub
4. Contact maintainers directly

Thank you for contributing to BotRS! 🚀
