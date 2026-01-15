#![macro_use]

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

#[macro_export]
#[collapse_debuginfo(yes)]
macro_rules! trace {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::maybe_log!(trace, $s $(, $x)*);
        }
    };
}

#[macro_export]
#[collapse_debuginfo(yes)]
macro_rules! debug {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::maybe_log!(debug, $s $(, $x)*);
        }
    };
}

#[macro_export]
#[collapse_debuginfo(yes)]
macro_rules! info {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::maybe_log!(info, $s $(, $x)*);
        }
    };
}

#[macro_export]
#[collapse_debuginfo(yes)]
macro_rules! warn {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::maybe_log!(warn, $s $(, $x)*);
        }
    };
}

#[macro_export]
#[collapse_debuginfo(yes)]
macro_rules! error {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::maybe_log!(error, $s $(, $x)*);
        }
    };
}
