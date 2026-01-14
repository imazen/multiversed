//! Test crate using multiversed with all features enabled
//!
//! This tests that all preset features can be enabled simultaneously without conflicts.

use multiversed::multiversed;

/// Function using default attribute (all presets active)
#[multiversed]
pub fn sum_array(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Function with explicit x86-64-v3 preset
#[multiversed("x86-64-v3")]
pub fn sum_array_v3(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Function with explicit x86-64-v4 preset (AVX-512)
#[multiversed("x86-64-v4")]
pub fn sum_array_v4(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Function with explicit aarch64-sve2 preset
#[multiversed("aarch64-sve2")]
pub fn sum_array_sve2(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Function with multiple explicit presets
#[multiversed("x86-64-v3", "aarch64-dotprod")]
pub fn sum_array_explicit(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Function with raw target string passthrough
#[multiversed("x86_64+avx2+fma")]
pub fn sum_array_raw(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_variants() {
        let data = [1.0f32, 2.0, 3.0, 4.0, 5.0];

        assert_eq!(sum_array(&data), 15.0);
        assert_eq!(sum_array_v3(&data), 15.0);
        assert_eq!(sum_array_v4(&data), 15.0);
        assert_eq!(sum_array_sve2(&data), 15.0);
        assert_eq!(sum_array_explicit(&data), 15.0);
        assert_eq!(sum_array_raw(&data), 15.0);
    }
}
