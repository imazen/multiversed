//! Test crate using multiversed with only x86 presets enabled
//!
//! Tests that x86-only configurations work correctly, and that the macro
//! generates appropriate cfg_attr for x86_64 architecture only.

use multiversed::multiversed;

/// Uses all three x86 presets from features (v2, v3, v4)
#[multiversed]
pub fn sum_all_x86(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Explicitly use v2 (SSE4.2 + POPCNT)
#[multiversed("x86-64-v2")]
pub fn sum_v2(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Explicitly use v3 (AVX2 + FMA + BMI)
#[multiversed("x86-64-v3")]
pub fn sum_v3(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Explicitly use v4 (AVX-512)
#[multiversed("x86-64-v4")]
pub fn sum_v4(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Mix of explicit presets
#[multiversed("x86-64-v2", "x86-64-v4")]
pub fn sum_v2_and_v4(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Raw x86_64 target string
#[multiversed("x86_64+sse4.2+popcnt")]
pub fn sum_raw_sse42(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [f32; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    const EXPECTED_SUM: f32 = 36.0;

    #[test]
    fn test_all_x86() {
        assert_eq!(sum_all_x86(&TEST_DATA), EXPECTED_SUM);
    }

    #[test]
    fn test_v2() {
        assert_eq!(sum_v2(&TEST_DATA), EXPECTED_SUM);
    }

    #[test]
    fn test_v3() {
        assert_eq!(sum_v3(&TEST_DATA), EXPECTED_SUM);
    }

    #[test]
    fn test_v4() {
        assert_eq!(sum_v4(&TEST_DATA), EXPECTED_SUM);
    }

    #[test]
    fn test_v2_and_v4() {
        assert_eq!(sum_v2_and_v4(&TEST_DATA), EXPECTED_SUM);
    }

    #[test]
    fn test_raw_sse42() {
        assert_eq!(sum_raw_sse42(&TEST_DATA), EXPECTED_SUM);
    }
}
