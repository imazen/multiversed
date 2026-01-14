//! Integration tests for multiversed macros.
//!
//! These tests verify that the generated code compiles and runs correctly
//! on the actual target architecture.

use multiversed::{multiversion, multiversion_aarch64, multiversion_x86};

// Basic test function
#[multiversion]
fn sum_f32(data: &[f32]) -> f32 {
    data.iter().sum()
}

// Test with explicit x86 targeting
#[multiversion_x86]
fn sum_f32_x86(data: &[f32]) -> f32 {
    data.iter().sum()
}

// Test with explicit aarch64 targeting
#[multiversion_aarch64]
fn sum_f32_aarch64(data: &[f32]) -> f32 {
    data.iter().sum()
}

// More complex function with multiple operations
#[multiversion]
fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

// Function with conditionals
#[multiversion]
fn max_f32(data: &[f32]) -> Option<f32> {
    data.iter().copied().reduce(f32::max)
}

// Test pub visibility
#[multiversion]
pub fn public_sum(data: &[f32]) -> f32 {
    data.iter().sum()
}

// Test with generics (should work, multiversion handles this)
#[multiversion]
fn sum_generic<T: std::iter::Sum + Copy>(data: &[T]) -> T {
    data.iter().copied().sum()
}

#[test]
fn test_basic_sum() {
    let data = [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let result = sum_f32(&data);
    assert!((result - 36.0).abs() < 0.001);
}

#[test]
fn test_x86_sum() {
    let data = [1.0f32, 2.0, 3.0, 4.0];
    let result = sum_f32_x86(&data);
    assert!((result - 10.0).abs() < 0.001);
}

#[test]
fn test_aarch64_sum() {
    let data = [1.0f32, 2.0, 3.0, 4.0];
    let result = sum_f32_aarch64(&data);
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
fn test_public_sum() {
    let data = [1.0f32, 2.0, 3.0];
    let result = public_sum(&data);
    assert!((result - 6.0).abs() < 0.001);
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

// Test larger data to exercise SIMD paths
#[test]
fn test_large_sum() {
    let data: Vec<f32> = (0..1024).map(|i| i as f32).collect();
    let result = sum_f32(&data);
    // Sum of 0..1024 = 1023 * 1024 / 2 = 523776
    assert!((result - 523776.0).abs() < 1.0);
}

#[test]
fn test_large_dot_product() {
    let a: Vec<f32> = (0..1024).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..1024).map(|i| (i * 2) as f32).collect();
    let result = dot_product(&a, &b);
    // Sum of i * (2*i) for i in 0..1024 = 2 * sum of i^2 = 2 * (n-1)*n*(2n-1)/6
    // = 2 * 1023 * 1024 * 2047 / 6 = 715_025_408
    let expected: f32 = (0..1024).map(|i| (i * 2 * i) as f32).sum();
    assert!((result - expected).abs() < 1000.0); // Allow for float precision
}
