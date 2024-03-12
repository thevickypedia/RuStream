#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        $crate::echo::functions::debug_func($msg, false)
    };
    ($msg:expr, $flag:expr) => {
        $crate::echo::functions::debug_func($msg, $flag)
    };
}

#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        $crate::echo::functions::info_func($msg, false)
    };
    ($msg:expr, $flag:expr) => {
        $crate::echo::functions::info_func($msg, $flag)
    };
}

#[macro_export]
macro_rules! warn {
    ($msg:expr) => {
        $crate::echo::functions::warn_func($msg, false)
    };
    ($msg:expr, $flag:expr) => {
        $crate::echo::functions::warn_func($msg, $flag)
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        $crate::echo::functions::error_func($msg, false)
    };
    ($msg:expr, $flag:expr) => {
        $crate::echo::functions::error_func($msg, $flag)
    };
}

#[macro_export]
macro_rules! critical {
    ($msg:expr) => {
        $crate::echo::functions::critical_func($msg, false)
    };
    ($msg:expr, $flag:expr) => {
        $crate::echo::functions::critical_func($msg, $flag)
    };
}
