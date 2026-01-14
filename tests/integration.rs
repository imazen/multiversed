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
// Explicit preset selection
// ============================================================================

#[multiversed("x86-64-v3")]
fn sum_x86_v3_only(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("aarch64-baseline")]
fn sum_aarch64_only(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("x86-64-v4", "aarch64-sve2")]
fn sum_high_tier(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("x86-64-v3", "x86-64-v4")]
fn sum_multi_x86(data: &[f32]) -> f32 {
    data.iter().sum()
}

#[multiversed("aarch64-baseline", "aarch64-dotprod", "aarch64-sve2")]
fn sum_multi_aarch64(data: &[f32]) -> f32 {
    data.iter().sum()
}

// ============================================================================
// Mixed presets and raw target strings
// ============================================================================

#[multiversed("x86-64-v3", "x86_64+avx2+fma")]
fn sum_mixed_x86(data: &[f32]) -> f32 {
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
fn sum_generic<T: std::iter::Sum + Copy>(data: &[T]) -> T {
    data.iter().copied().sum()
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
fn test_x86_v3_only() {
    let data = [1.0f32, 2.0, 3.0, 4.0];
    let result = sum_x86_v3_only(&data);
    assert!((result - 10.0).abs() < 0.001);
}

#[test]
fn test_aarch64_only() {
    let data = [1.0f32, 2.0, 3.0, 4.0];
    let result = sum_aarch64_only(&data);
    assert!((result - 10.0).abs() < 0.001);
}

#[test]
fn test_high_tier() {
    let data = [1.0f32, 2.0, 3.0, 4.0];
    let result = sum_high_tier(&data);
    assert!((result - 10.0).abs() < 0.001);
}

#[test]
fn test_multi_x86() {
    let data = [1.0f32, 2.0, 3.0, 4.0];
    let result = sum_multi_x86(&data);
    assert!((result - 10.0).abs() < 0.001);
}

#[test]
fn test_multi_aarch64() {
    let data = [1.0f32, 2.0, 3.0, 4.0];
    let result = sum_multi_aarch64(&data);
    assert!((result - 10.0).abs() < 0.001);
}

#[test]
fn test_mixed_x86() {
    let data = [1.0f32, 2.0, 3.0, 4.0];
    let result = sum_mixed_x86(&data);
    assert!((result - 10.0).abs() < 0.001);
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
