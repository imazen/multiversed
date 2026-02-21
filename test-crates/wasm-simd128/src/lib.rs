//! Test crate verifying multiversed works on wasm32-wasip1.
//!
//! On wasm32, multiversion elides itself — functions are passthrough.
//! This test confirms: compilation succeeds, functions produce correct
//! results, and simd128 target_feature is detectable at compile time.

use multiversed::multiversed;

// ============================================================================
// Functions using presets (all become passthrough on wasm32)
// ============================================================================

#[multiversed]
pub fn sum_default(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("x86-64-v3")]
pub fn sum_x86_v3(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("arm64-v2")]
pub fn sum_arm64_v2(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("wasm32-simd128")]
pub fn sum_wasm(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("x86-64-v3", "arm64-v2", "wasm32-simd128")]
pub fn sum_all_archs(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed]
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

// ============================================================================
// Runtime report
// ============================================================================

pub fn report_wasm_features() {
    println!("=== wasm32 multiversed report ===");
    println!();
    println!("  target_arch:     {}", std::env::consts::ARCH);

    let has_simd128 = cfg!(target_feature = "simd128");
    println!(
        "  wasm32-simd128:  {}",
        if has_simd128 {
            "YES (compiled with simd128)"
        } else {
            "no (compiled without simd128)"
        }
    );
    println!();
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: [f32; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

    #[test]
    fn test_default() {
        assert_eq!(sum_default(&DATA), 36.0);
    }

    #[test]
    fn test_x86_preset_on_wasm() {
        // x86 preset becomes passthrough on wasm32
        assert_eq!(sum_x86_v3(&DATA), 36.0);
    }

    #[test]
    fn test_arm64_preset_on_wasm() {
        // arm64 preset becomes passthrough on wasm32
        assert_eq!(sum_arm64_v2(&DATA), 36.0);
    }

    #[test]
    fn test_wasm_preset() {
        assert_eq!(sum_wasm(&DATA), 36.0);
    }

    #[test]
    fn test_all_archs() {
        assert_eq!(sum_all_archs(&DATA), 36.0);
    }

    #[test]
    fn test_dot_product() {
        let a = [1.0f32, 2.0, 3.0, 4.0];
        let b = [2.0f32, 3.0, 4.0, 5.0];
        assert_eq!(dot_product(&a, &b), 40.0);
    }

    #[test]
    fn test_simd128_feature_detection() {
        report_wasm_features();
        // On wasm32, this checks compile-time feature
        if cfg!(target_arch = "wasm32") {
            // We're actually running on wasm — check makes sense
            let has_simd128 = cfg!(target_feature = "simd128");
            println!("  simd128 enabled: {has_simd128}");
        }
    }
}
