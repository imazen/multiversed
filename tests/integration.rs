//! Integration tests for multiversed macros.
//!
//! These tests verify that the generated code compiles and runs correctly
//! on the actual target architecture.

use multiversed::multiversed;

// ============================================================================
// Basic usage - no arguments (uses cargo feature defaults)
// ============================================================================

#[multiversed]
fn sum_default(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed]
pub fn public_sum(data: &[f32]) -> f32 {
    data.iter().sum()
}

// ============================================================================
// Explicit x86 preset selection
// ============================================================================

#[multiversed("x86-64-v2")]
fn sum_x86_v2(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("x86-64-v3")]
fn sum_x86_v3(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("x86-64-v4")]
fn sum_x86_v4(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("x86-64-v3", "x86-64-v4")]
fn sum_multi_x86(data: &[f32]) -> f32 {
    data.iter().sum()
}

// ============================================================================
// Explicit aarch64 preset selection
// ============================================================================

#[multiversed("arm64")]
fn sum_arm64(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("arm64-v2")]
fn sum_arm64_v2(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("arm64-v3")]
fn sum_arm64_v3(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("arm64-v3", "arm64-v2", "arm64")]
fn sum_arm64_tiered(data: &[f32]) -> f32 {
    data.iter().sum()
}

// Raw target string for custom feature combinations
#[multiversed("aarch64+neon+dotprod")]
fn sum_aarch64_dotprod(data: &[f32]) -> f32 {
    data.iter().sum()
}

// ============================================================================
// wasm32 preset (ignored by multiversion, just passthrough)
// ============================================================================

// wasm32-simd128 is silently ignored since multiversion doesn't support wasm32
#[multiversed("wasm32-simd128")]
fn sum_wasm_simd128(data: &[f32]) -> f32 {
    data.iter().sum()
}

// ============================================================================
// Cross-architecture combinations
// ============================================================================

#[multiversed("x86-64-v4", "arm64")]
fn sum_high_tier(data: &[f32]) -> f32 {
    data.iter().sum()
}

// wasm32-simd128 is silently filtered out
#[multiversed("x86-64-v3", "arm64", "wasm32-simd128")]
fn sum_all_archs(data: &[f32]) -> f32 {
    data.iter().sum()
}

// ============================================================================
// Mixed presets and raw target strings
// ============================================================================

#[multiversed("x86-64-v3", "x86_64+avx2+fma")]
fn sum_mixed_x86(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("arm64", "aarch64+neon")]
fn sum_mixed_aarch64(data: &[f32]) -> f32 {
    data.iter().sum()
}

// ============================================================================
// More complex functions
// ============================================================================

#[multiversed]
fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

#[multiversed]
fn max_f32(data: &[f32]) -> Option<f32> {
    data.iter().copied().reduce(f32::max)
}

#[multiversed]
fn min_f32(data: &[f32]) -> Option<f32> {
    data.iter().copied().reduce(f32::min)
}

#[multiversed]
fn sum_generic<T: std::iter::Sum + Copy>(data: &[T]) -> T {
    data.iter().copied().sum()
}

#[multiversed]
fn mean_f32(data: &[f32]) -> f32 {
    if data.is_empty() {
        return 0.0;
    }
    data.iter().sum::<f32>() / data.len() as f32
}

#[multiversed]
fn variance_f32(data: &[f32]) -> f32 {
    if data.is_empty() {
        return 0.0;
    }
    let mean = data.iter().sum::<f32>() / data.len() as f32;
    data.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / data.len() as f32
}

// ============================================================================
// Tests
// ============================================================================

#[test]
fn test_default_sum() {
    let data = [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let result = sum_default(&data);
    assert!((result - 36.0).abs() < 0.001);
}

#[test]
fn test_public_sum() {
    let data = [1.0f32, 2.0, 3.0];
    let result = public_sum(&data);
    assert!((result - 6.0).abs() < 0.001);
}

#[test]
fn test_x86_presets() {
    let data = [1.0f32, 2.0, 3.0, 4.0];
    assert!((sum_x86_v2(&data) - 10.0).abs() < 0.001);
    assert!((sum_x86_v3(&data) - 10.0).abs() < 0.001);
    assert!((sum_x86_v4(&data) - 10.0).abs() < 0.001);
    assert!((sum_multi_x86(&data) - 10.0).abs() < 0.001);
}

#[test]
fn test_aarch64_presets() {
    let data = [1.0f32, 2.0, 3.0, 4.0];
    assert!((sum_arm64(&data) - 10.0).abs() < 0.001);
    assert!((sum_arm64_v2(&data) - 10.0).abs() < 0.001);
    assert!((sum_arm64_v3(&data) - 10.0).abs() < 0.001);
    assert!((sum_arm64_tiered(&data) - 10.0).abs() < 0.001);
    assert!((sum_aarch64_dotprod(&data) - 10.0).abs() < 0.001);
}

#[test]
fn test_wasm_preset() {
    let data = [1.0f32, 2.0, 3.0, 4.0];
    assert!((sum_wasm_simd128(&data) - 10.0).abs() < 0.001);
}

#[test]
fn test_cross_arch() {
    let data = [1.0f32, 2.0, 3.0, 4.0];
    assert!((sum_high_tier(&data) - 10.0).abs() < 0.001);
    assert!((sum_all_archs(&data) - 10.0).abs() < 0.001);
}

#[test]
fn test_mixed_targets() {
    let data = [1.0f32, 2.0, 3.0, 4.0];
    assert!((sum_mixed_x86(&data) - 10.0).abs() < 0.001);
    assert!((sum_mixed_aarch64(&data) - 10.0).abs() < 0.001);
}

#[test]
fn test_dot_product() {
    let a = [1.0f32, 2.0, 3.0, 4.0];
    let b = [2.0f32, 3.0, 4.0, 5.0];
    let result = dot_product(&a, &b);
    // 1*2 + 2*3 + 3*4 + 4*5 = 2 + 6 + 12 + 20 = 40
    assert!((result - 40.0).abs() < 0.001);
}

#[test]
fn test_max() {
    let data = [1.0f32, 5.0, 3.0, 7.0, 2.0];
    let result = max_f32(&data);
    assert_eq!(result, Some(7.0));
}

#[test]
fn test_min() {
    let data = [1.0f32, 5.0, 3.0, 7.0, 2.0];
    let result = min_f32(&data);
    assert_eq!(result, Some(1.0));
}

#[test]
fn test_empty_max() {
    let data: [f32; 0] = [];
    let result = max_f32(&data);
    assert_eq!(result, None);
}

#[test]
fn test_generic_sum_f32() {
    let data = [1.0f32, 2.0, 3.0];
    let result: f32 = sum_generic(&data);
    assert!((result - 6.0).abs() < 0.001);
}

#[test]
fn test_generic_sum_i32() {
    let data = [1i32, 2, 3, 4, 5];
    let result: i32 = sum_generic(&data);
    assert_eq!(result, 15);
}

#[test]
fn test_mean() {
    let data = [1.0f32, 2.0, 3.0, 4.0, 5.0];
    let result = mean_f32(&data);
    assert!((result - 3.0).abs() < 0.001);
}

#[test]
fn test_mean_empty() {
    let data: [f32; 0] = [];
    let result = mean_f32(&data);
    assert!((result - 0.0).abs() < 0.001);
}

#[test]
fn test_variance() {
    let data = [2.0f32, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let result = variance_f32(&data);
    // Mean = 5, Variance = 4
    assert!((result - 4.0).abs() < 0.001);
}

#[test]
fn test_large_sum() {
    let data: Vec<f32> = (0..1024).map(|i| i as f32).collect();
    let result = sum_default(&data);
    // Sum of 0..1024 = 1023 * 1024 / 2 = 523776
    assert!((result - 523776.0).abs() < 1.0);
}

#[test]
fn test_large_dot_product() {
    let a: Vec<f32> = (0..1024).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..1024).map(|i| (i * 2) as f32).collect();
    let result = dot_product(&a, &b);
    let expected: f32 = (0..1024).map(|i| (i * 2 * i) as f32).sum();
    assert!((result - expected).abs() < 1000.0);
}
