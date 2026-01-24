#![macro_use]

#[doc(hidden)]
#[macro_export]
macro_rules! maybe_log {
    ($level:ident, $s:literal $(, $x:expr)* $(,)?) => {
        {
        #[cfg(feature = "defmt")]
        ::defmt::$level!($s $(, $x)*);
        #[cfg(not(feature="defmt"))]
        let _ = ($( & $x ),*);
        }
    };
}

/// Expands to `defmt::assert_eq!` if the `defmt` feature is enabled, uses core impl otherwise.
#[macro_export]
#[collapse_debuginfo(yes)]
macro_rules! assert_eq {
    ($($x:tt)*) => {
        {
            #[cfg(not(feature = "defmt"))]
            ::core::assert_eq!($($x)*);
            #[cfg(feature = "defmt")]
            ::defmt::assert_eq!($($x)*);
        }
    };
}

/// Log at the `trace` level if the `defmat` feature is enabled. No-op otherwise.
#[macro_export]
#[collapse_debuginfo(yes)]
macro_rules! trace {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::maybe_log!(trace, $s $(, $x)*);
        }
    };
}

/// Log at the `debug` level if the `defmat` feature is enabled. No-op otherwise.
#[macro_export]
#[collapse_debuginfo(yes)]
macro_rules! debug {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::maybe_log!(debug, $s $(, $x)*);
        }
    };
}

/// Log at the `info` level if the `defmat` feature is enabled. No-op otherwise.
#[macro_export]
#[collapse_debuginfo(yes)]
macro_rules! info {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::maybe_log!(info, $s $(, $x)*);
        }
    };
}

/// Log at the `warn` level if the `defmat` feature is enabled. No-op otherwise.
#[macro_export]
#[collapse_debuginfo(yes)]
macro_rules! warn {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::maybe_log!(warn, $s $(, $x)*);
        }
    };
}

/// Log at the `error` level if the `defmat` feature is enabled. No-op otherwise.
#[macro_export]
#[collapse_debuginfo(yes)]
macro_rules! error {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::maybe_log!(error, $s $(, $x)*);
        }
    };
}
