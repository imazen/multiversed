//! Test crate using multiversed with only aarch64 presets enabled
//!
//! Tests that aarch64-only configurations work correctly, and that the macro
//! generates appropriate cfg_attr for aarch64 architecture only.
//! Note: baseline NEON is implicit - these presets are all above baseline.

use multiversed::multiversed;

/// Uses all aarch64 presets from features
#[multiversed]
pub fn sum_all_aarch64(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Explicitly use basic (dotprod + fp16)
#[multiversed("aarch64-basic")]
pub fn sum_basic(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Explicitly use v84 (sha3 + fcma)
#[multiversed("aarch64-v84")]
pub fn sum_v84(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Explicitly use sve (SVE + i8mm + bf16, Graviton3)
#[multiversed("aarch64-sve")]
pub fn sum_sve(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Explicitly use sve2 (SVE2 + i8mm + bf16, Graviton4)
#[multiversed("aarch64-sve2")]
pub fn sum_sve2(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Mix of explicit presets
#[multiversed("aarch64-basic", "aarch64-sve2")]
pub fn sum_basic_and_sve2(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Raw aarch64 target string
#[multiversed("aarch64+neon+aes")]
pub fn sum_raw_neon(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [f32; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    const EXPECTED_SUM: f32 = 36.0;

    #[test]
    fn test_all_aarch64() {
        assert_eq!(sum_all_aarch64(&TEST_DATA), EXPECTED_SUM);
    }

    #[test]
    fn test_basic() {
        assert_eq!(sum_basic(&TEST_DATA), EXPECTED_SUM);
    }

    #[test]
    fn test_v84() {
        assert_eq!(sum_v84(&TEST_DATA), EXPECTED_SUM);
    }

    #[test]
    fn test_sve() {
        assert_eq!(sum_sve(&TEST_DATA), EXPECTED_SUM);
    }

    #[test]
    fn test_sve2() {
        assert_eq!(sum_sve2(&TEST_DATA), EXPECTED_SUM);
    }

    #[test]
    fn test_basic_and_sve2() {
        assert_eq!(sum_basic_and_sve2(&TEST_DATA), EXPECTED_SUM);
    }

    #[test]
    fn test_raw_neon() {
        assert_eq!(sum_raw_neon(&TEST_DATA), EXPECTED_SUM);
    }
}
