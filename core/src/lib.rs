pub mod percentage {
    /// A newtype for representing a percentage value between 0.0 and 1.0 (inclusive).
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
    pub struct T(f64);

    impl T {
        /// Creates a new `Percentage` from an `f64`.
        /// Returns `Ok(Percentage)` if the value is within [0.0, 1.0], otherwise returns an `Err` with a message.
        pub fn new(value: f64) -> Result<Self, String> {
            if value >= 0.0 && value <= 1.0 {
                Ok(T(value))
            } else {
                Err(format!(
                    "Percentage value must be between 0.0 and 1.0, got {}",
                    value
                ))
            }
        }

        /// Returns the inner `f64` value of the `Percentage`.
        pub fn as_f64(&T(value): &Self) -> f64 {
            value
        }
    }

    impl std::fmt::Display for T {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let T(value) = *self;
            write!(f, "{:.2}%", value * 100.0)
        }
    }
}
