//! Test crate using multiversed with all features enabled
//!
//! This tests that all preset features can be enabled simultaneously without conflicts.

use multiversed::multiversed;

// ============================================================================
// Default (uses cargo feature presets)
// ============================================================================

#[multiversed]
pub fn sum_default(data: &[f32]) -> f32 {
    data.iter().sum()
}

// ============================================================================
// Named presets - x86
// ============================================================================

#[multiversed("x86-64-v2")]
pub fn sum_x86_v2(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("x86-64-v3")]
pub fn sum_x86_v3(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("x86-64-v4")]
pub fn sum_x86_v4(data: &[f32]) -> f32 {
    data.iter().sum()
}

// ============================================================================
// Named presets - aarch64
// ============================================================================

#[multiversed("aarch64-basic")]
pub fn sum_aarch64_basic(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("aarch64-v84")]
pub fn sum_aarch64_v84(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("aarch64-sve")]
pub fn sum_aarch64_sve(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("aarch64-sve2")]
pub fn sum_aarch64_sve2(data: &[f32]) -> f32 {
    data.iter().sum()
}

// ============================================================================
// Multiple tiers (runtime picks best)
// ============================================================================

#[multiversed("x86-64-v4", "x86-64-v3", "x86-64-v2")]
pub fn sum_x86_tiered(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("aarch64-sve2", "aarch64-sve", "aarch64-v84", "aarch64-basic")]
pub fn sum_aarch64_tiered(data: &[f32]) -> f32 {
    data.iter().sum()
}

// ============================================================================
// Raw target strings (passthrough)
// ============================================================================

#[multiversed("x86_64+avx2+fma")]
pub fn sum_raw_avx2(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("x86_64+avx512f+avx512vbmi2")]
pub fn sum_raw_avx512(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("aarch64+neon+dotprod")]
pub fn sum_raw_neon(data: &[f32]) -> f32 {
    data.iter().sum()
}

// ============================================================================
// Mixed presets and raw strings
// ============================================================================

#[multiversed("x86-64-v3", "x86_64+avx512f", "aarch64-basic")]
pub fn sum_mixed(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let data = [1.0f32, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(sum_default(&data), 15.0);
    }

    #[test]
    fn test_x86_presets() {
        let data = [1.0f32, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(sum_x86_v2(&data), 15.0);
        assert_eq!(sum_x86_v3(&data), 15.0);
        assert_eq!(sum_x86_v4(&data), 15.0);
    }

    #[test]
    fn test_aarch64_presets() {
        let data = [1.0f32, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(sum_aarch64_basic(&data), 15.0);
        assert_eq!(sum_aarch64_v84(&data), 15.0);
        assert_eq!(sum_aarch64_sve(&data), 15.0);
        assert_eq!(sum_aarch64_sve2(&data), 15.0);
    }

    #[test]
    fn test_tiered() {
        let data = [1.0f32, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(sum_x86_tiered(&data), 15.0);
        assert_eq!(sum_aarch64_tiered(&data), 15.0);
    }

    #[test]
    fn test_raw_strings() {
        let data = [1.0f32, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(sum_raw_avx2(&data), 15.0);
        assert_eq!(sum_raw_avx512(&data), 15.0);
        assert_eq!(sum_raw_neon(&data), 15.0);
    }

    #[test]
    fn test_mixed() {
        let data = [1.0f32, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(sum_mixed(&data), 15.0);
    }
}
