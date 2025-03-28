#[macro_export(local_inner_macros)]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log::error!(target: $target, $($arg)+)

    );
    ($($arg:tt)+) => (
        $crate::log::error!($($arg)+);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log::warn!(target: $target, $($arg)+)

    );
    ($($arg:tt)+) => (
        $crate::log::warn!($($arg)+);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log::info!(target: $target, $($arg)+)

    );
    ($($arg:tt)+) => (
        $crate::log::info!($($arg)+);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log::debug!(target: $target, $($arg)+)

    );
    ($($arg:tt)+) => (
        $crate::log::debug!($($arg)+);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log::trace!(target: $target, $($arg)+)

    );
    ($($arg:tt)+) => (
        $crate::log::trace!($($arg)+);
    )
}

#[cfg(feature = "target")]
#[macro_export(local_inner_macros)]
macro_rules! log_target {
    ($($x:expr),+ $(,)?) => {
        $(
            $crate::log_target_derive!($x);
        )+
    };
    ($arg:expr) => {
        $crate::log_target_derive!($arg);
    };
}

#[macro_export]
macro_rules! quick {
    () => {
        $crate::quick_log_level::<_, &str>($crate::Level::Debug, None).unwrap()
    };
    ($level:expr) => {{
        $crate::quick_log_level::<_, &str>($level, None).unwrap()
    }};
    ($level:expr,$path:expr) => {{
        $crate::quick_log_level($level, Some($path)).unwrap()
    }};
}

#[cfg(feature = "println")]
#[macro_export]
macro_rules! println {
    ($($arg:tt)+) => (

        if !$crate::PRINTLN_INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            if !$crate::PRINTLN_INITIALIZED.swap(true, std::sync::atomic::Ordering::SeqCst) {
                $crate::console($crate::Level::Debug).unwrap();
            }
        }

        $crate::log::debug!($($arg)+);
    )
}
