//! Test crate using multiversed with no features enabled (default-features = false)
//!
//! With no presets enabled, functions using the bare `#[multiversed]` attribute
//! become passthroughs. Only explicit preset arguments or raw target strings
//! will generate multiversion code.

use multiversed::multiversed;

/// With no features enabled, this should be a passthrough (no multiversioning)
#[multiversed]
pub fn passthrough_function(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Explicit preset should still work even without feature flags
#[multiversed("x86-64-v3")]
pub fn explicit_preset(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Raw target string should still work
#[multiversed("x86_64+avx2+fma")]
pub fn raw_target(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passthrough() {
        let data = [1.0f32, 2.0, 3.0, 4.0];
        assert_eq!(passthrough_function(&data), 10.0);
    }

    #[test]
    fn test_explicit_preset() {
        let data = [1.0f32, 2.0, 3.0, 4.0];
        assert_eq!(explicit_preset(&data), 10.0);
    }

    #[test]
    fn test_raw_target() {
        let data = [1.0f32, 2.0, 3.0, 4.0];
        assert_eq!(raw_target(&data), 10.0);
    }
}
