use std::sync::{LazyLock, RwLock};
use crate::log_trait::Logger;

#[cfg(feature = "default_logger")]
use crate::impls::console_logger::ConsoleLogger;

/// グローバルロガー
/// 
/// `GLOBAL_LOGGER` は遅延初期化され、アクセス時に一度だけ初期化されます。

pub(crate) static GLOBAL_LOGGER: LazyLock<RwLock<Box<dyn Logger>>> = LazyLock::new(|| {
    #[cfg(feature = "default_logger")]
    {
        RwLock::new(Box::new(ConsoleLogger::new()))
    }
});

