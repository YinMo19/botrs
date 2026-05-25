//! Botgo-style logging facade backed by `tracing`.

#![allow(non_snake_case, non_upper_case_globals)]

use std::fmt;
use std::sync::{LazyLock, RwLock};

/// Logger interface used by this framework.
pub trait Logger: Send + Sync {
    fn debug(&self, args: fmt::Arguments<'_>);
    fn info(&self, args: fmt::Arguments<'_>);
    fn warn(&self, args: fmt::Arguments<'_>);
    fn error(&self, args: fmt::Arguments<'_>);
    fn sync(&self) -> crate::Result<()>;

    fn Debug(&self, args: fmt::Arguments<'_>) {
        self.debug(args);
    }

    fn Info(&self, args: fmt::Arguments<'_>) {
        self.info(args);
    }

    fn Warn(&self, args: fmt::Arguments<'_>) {
        self.warn(args);
    }

    fn Error(&self, args: fmt::Arguments<'_>) {
        self.error(args);
    }

    fn Debugf(&self, args: fmt::Arguments<'_>) {
        self.debug(args);
    }

    fn Infof(&self, args: fmt::Arguments<'_>) {
        self.info(args);
    }

    fn Warnf(&self, args: fmt::Arguments<'_>) {
        self.warn(args);
    }

    fn Errorf(&self, args: fmt::Arguments<'_>) {
        self.error(args);
    }

    fn Sync(&self) -> crate::Result<()> {
        self.sync()
    }
}

#[derive(Debug, Default)]
pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn debug(&self, args: fmt::Arguments<'_>) {
        tracing::debug!("{}", args);
    }

    fn info(&self, args: fmt::Arguments<'_>) {
        tracing::info!("{}", args);
    }

    fn warn(&self, args: fmt::Arguments<'_>) {
        tracing::warn!("{}", args);
    }

    fn error(&self, args: fmt::Arguments<'_>) {
        tracing::error!("{}", args);
    }

    fn sync(&self) -> crate::Result<()> {
        Ok(())
    }
}

pub static DefaultLogger: LazyLock<RwLock<Box<dyn Logger>>> =
    LazyLock::new(|| RwLock::new(Box::<ConsoleLogger>::default()));

pub fn set_logger(logger: impl Logger + 'static) {
    *DefaultLogger.write().expect("default logger lock poisoned") = Box::new(logger);
}

pub fn with_logger<R>(f: impl FnOnce(&dyn Logger) -> R) -> R {
    let logger = DefaultLogger.read().expect("default logger lock poisoned");
    f(logger.as_ref())
}

pub fn Debug(message: impl fmt::Display) {
    with_logger(|logger| logger.debug(format_args!("{}", message)));
}

pub fn Info(message: impl fmt::Display) {
    with_logger(|logger| logger.info(format_args!("{}", message)));
}

pub fn Warn(message: impl fmt::Display) {
    with_logger(|logger| logger.warn(format_args!("{}", message)));
}

pub fn Error(message: impl fmt::Display) {
    with_logger(|logger| logger.error(format_args!("{}", message)));
}

pub fn Debugf(args: fmt::Arguments<'_>) {
    with_logger(|logger| logger.debug(args));
}

pub fn Infof(args: fmt::Arguments<'_>) {
    with_logger(|logger| logger.info(args));
}

pub fn Warnf(args: fmt::Arguments<'_>) {
    with_logger(|logger| logger.warn(args));
}

pub fn Errorf(args: fmt::Arguments<'_>) {
    with_logger(|logger| logger.error(args));
}

pub fn Sync() {
    let _ = with_logger(|logger| logger.sync());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[derive(Default)]
    struct TestLogger {
        messages: Arc<Mutex<Vec<String>>>,
    }

    impl Logger for TestLogger {
        fn debug(&self, args: fmt::Arguments<'_>) {
            self.messages.lock().unwrap().push(args.to_string());
        }

        fn info(&self, args: fmt::Arguments<'_>) {
            self.messages.lock().unwrap().push(args.to_string());
        }

        fn warn(&self, args: fmt::Arguments<'_>) {
            self.messages.lock().unwrap().push(args.to_string());
        }

        fn error(&self, args: fmt::Arguments<'_>) {
            self.messages.lock().unwrap().push(args.to_string());
        }

        fn sync(&self) -> crate::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn log_facade_accepts_custom_logger() {
        let messages = Arc::new(Mutex::new(Vec::new()));
        set_logger(TestLogger {
            messages: messages.clone(),
        });

        Info("hello");
        Debugf(format_args!("value={}", 42));
        Sync();

        assert_eq!(
            messages.lock().unwrap().clone(),
            vec!["hello".to_string(), "value=42".to_string()]
        );
        set_logger(ConsoleLogger);
    }
}
