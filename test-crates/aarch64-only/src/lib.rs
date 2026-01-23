//! Test crate using multiversed with only aarch64 presets enabled
//!
//! Tests that aarch64-only configurations work correctly, and that the macro
//! generates appropriate cfg_attr for aarch64 architecture only.

use multiversed::multiversed;

/// Uses arm64 preset from features
#[multiversed]
pub fn sum_all_aarch64(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Explicitly use arm64 (NEON + FP16)
#[multiversed("arm64")]
pub fn sum_arm64(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Raw aarch64 target string with dotprod
#[multiversed("aarch64+neon+dotprod")]
pub fn sum_dotprod(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Raw aarch64 target string with SHA3
#[multiversed("aarch64+neon+sha3")]
pub fn sum_sha3(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Raw aarch64 target string with SVE
#[multiversed("aarch64+neon+sve")]
pub fn sum_sve(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Raw aarch64 target string with SVE2
#[multiversed("aarch64+neon+sve2")]
pub fn sum_sve2(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Mix of arm64 preset and raw SVE2
#[multiversed("arm64", "aarch64+neon+sve2")]
pub fn sum_arm64_and_sve2(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Raw aarch64 target string with AES
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
    fn test_arm64() {
        assert_eq!(sum_arm64(&TEST_DATA), EXPECTED_SUM);
    }

    #[test]
    fn test_dotprod() {
        assert_eq!(sum_dotprod(&TEST_DATA), EXPECTED_SUM);
    }

    #[test]
    fn test_sha3() {
        assert_eq!(sum_sha3(&TEST_DATA), EXPECTED_SUM);
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
    fn test_arm64_and_sve2() {
        assert_eq!(sum_arm64_and_sve2(&TEST_DATA), EXPECTED_SUM);
    }

    #[test]
    fn test_raw_neon() {
        assert_eq!(sum_raw_neon(&TEST_DATA), EXPECTED_SUM);
    }
}
