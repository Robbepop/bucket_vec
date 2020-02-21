//! Utility module to help with `no_std` Rust not supporting certain float operations.

/// Trait to either forward implementation to `std` library or `libm` for `no_std`.
pub trait FloatExt {
    /// Calculates the absolute value for the float.
    fn abs(self) -> Self;
    /// Rounds the float down to the next natural representable float.
    fn floor(self) -> Self;
    /// Rounds teh float up to the next natural representable float.
    fn ceil(self) -> Self;
    /// Calculates the natural logarithm of the float to the base.
    fn log(self, base: Self) -> Self;
    /// Calculates the power of the float to the integer exponent.
    fn powi(self, exp: i32) -> Self;
}

#[cfg(feature = "std")]
impl FloatExt for f64 {
    fn abs(self) -> Self {
        f64::abs(self)
    }

    fn floor(self) -> Self {
        f64::floor(self)
    }

    fn ceil(self) -> Self {
        f64::ceil(self)
    }

    fn log(self, base: Self) -> Self {
        f64::log(self, base)
    }

    fn powi(self, exp: i32) -> Self {
        f64::powi(self, exp)
    }
}

#[cfg(not(feature = "std"))]
impl FloatExt for f64 {
    fn abs(self) -> Self {
        libm::fabs(self)
    }

    fn floor(self) -> Self {
        libm::floor(self)
    }

    fn ceil(self) -> Self {
        libm::ceil(self)
    }

    fn log(self, base: Self) -> Self {
        libm::log(self) / libm::log(base)
    }

    fn powi(self, exp: i32) -> Self {
        libm::powi(self as f32, exp) as f64
    }
}
