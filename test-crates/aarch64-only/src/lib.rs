//! Test crate using multiversed with only aarch64 presets enabled
//!
//! Tests that aarch64-only configurations work correctly, and that the macro
//! generates appropriate cfg_attr for aarch64 architecture only.

use multiversed::multiversed;

/// Uses all four aarch64 presets from features
#[multiversed]
pub fn sum_all_aarch64(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Explicitly use baseline (NEON + crypto)
#[multiversed("aarch64-baseline")]
pub fn sum_baseline(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Explicitly use dotprod (dotprod + fp16)
#[multiversed("aarch64-dotprod")]
pub fn sum_dotprod(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Explicitly use crypto-ext (sha3 + fcma)
#[multiversed("aarch64-crypto-ext")]
pub fn sum_crypto_ext(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Explicitly use sve2 (SVE2 + i8mm + bf16)
#[multiversed("aarch64-sve2")]
pub fn sum_sve2(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Mix of explicit presets
#[multiversed("aarch64-baseline", "aarch64-sve2")]
pub fn sum_baseline_and_sve2(data: &[f32]) -> f32 {
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
    fn test_baseline() {
        assert_eq!(sum_baseline(&TEST_DATA), EXPECTED_SUM);
    }

    #[test]
    fn test_dotprod() {
        assert_eq!(sum_dotprod(&TEST_DATA), EXPECTED_SUM);
    }

    #[test]
    fn test_crypto_ext() {
        assert_eq!(sum_crypto_ext(&TEST_DATA), EXPECTED_SUM);
    }

    #[test]
    fn test_sve2() {
        assert_eq!(sum_sve2(&TEST_DATA), EXPECTED_SUM);
    }

    #[test]
    fn test_baseline_and_sve2() {
        assert_eq!(sum_baseline_and_sve2(&TEST_DATA), EXPECTED_SUM);
    }

    #[test]
    fn test_raw_neon() {
        assert_eq!(sum_raw_neon(&TEST_DATA), EXPECTED_SUM);
    }
}
