//! Test crate using multiversed with force-disable feature
//!
//! When force-disable is enabled, ALL `#[multiversed]` attributes become pure
//! passthroughs - no multiversion code is generated, regardless of other features
//! or explicit arguments. This is useful for debugging or faster builds.

use multiversed::multiversed;

/// With force-disable, this is a plain function (no multiversion)
#[multiversed]
pub fn sum_array(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Even with explicit presets, force-disable makes it a passthrough
#[multiversed("x86-64-v3", "aarch64-basic")]
pub fn sum_with_presets(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Raw targets are also ignored with force-disable
#[multiversed("x86_64+avx2+fma+bmi2")]
pub fn sum_with_raw(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Complex function signature - still just a passthrough
#[multiversed("x86-64-v4")]
pub fn complex_operation<T: AsRef<[f32]>>(input: T, scale: f32) -> Vec<f32> {
    input.as_ref().iter().map(|x| x * scale).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_sum() {
        let data = [1.0f32, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(sum_array(&data), 15.0);
    }

    #[test]
    fn test_with_presets() {
        let data = [2.0f32, 4.0, 6.0, 8.0];
        assert_eq!(sum_with_presets(&data), 20.0);
    }

    #[test]
    fn test_with_raw() {
        let data = [1.5f32, 2.5, 3.5];
        assert_eq!(sum_with_raw(&data), 7.5);
    }

    #[test]
    fn test_complex() {
        let input = vec![1.0f32, 2.0, 3.0];
        let result = complex_operation(input, 2.0);
        assert_eq!(result, vec![2.0, 4.0, 6.0]);
    }
}
