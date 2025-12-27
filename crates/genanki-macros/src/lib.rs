//! Declarative macros for genanki-rs
//!
//! This crate provides declarative macros to reduce boilerplate code.

/// Declarative macro for simple From implementations
///
/// # Example
/// ```rust
/// use genanki_macros::impl_from_simple;
///
/// struct Field {
///     name: String,
/// }
///
/// struct Fld {
///     name: String,
/// }
///
/// impl_from_simple!(Field, Fld, { name });
/// ```
#[macro_export]
macro_rules! impl_from_simple {
    ($from:ty, $to:ty, { $($field:ident),* $(,)? }) => {
        impl From<$from> for $to {
            fn from(value: $from) -> Self {
                Self {
                    $(
                        $field: value.$field.clone(),
                    )*
                }
            }
        }
    };
}

/// Declarative macro for creating field configuration with defaults
#[macro_export]
macro_rules! field_defaults {
    () => {
        {
            sticky: false,
            rtl: false,
            font: "Liberation Sans".to_string(),
            size: 20,
        }
    };
}

/// Declarative macro for builder field setters
#[macro_export]
macro_rules! builder_setters {
    ($struct_name:ident, { $($field:ident: $ty:ty),* $(,)? }) => {
        $(
            pub fn $field(mut self, value: $ty) -> Self {
                self.$field = Some(value);
                self
            }
        )*
    };
}

/// Declarative macro for database parameter arrays
#[macro_export]
macro_rules! db_params {
    ($($param:expr),* $(,)?) => {
        params![$($param),*]
    };
}

/// Declarative macro for creating SQL statements with placeholders
#[macro_export]
macro_rules! sql_stmt {
    ($table:ident, $($field:ident),* $(,)?) => {
        format!(
            "INSERT INTO {} VALUES({});",
            stringify!($table),
            &vec!["?"; $crate::count!($($field),*)].join(",")
        )
    };
}

/// Internal macro to count items
#[macro_export]
macro_rules! count {
    () => { 0 };
    ($x:tt $(, $rest:tt)*) => { 1 + $crate::count!($($rest),*) };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_count_macro() {
        assert_eq!(count![], 0);
        assert_eq!(count![a], 1);
        assert_eq!(count![a, b, c], 3);
    }
}
