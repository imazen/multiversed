//! Benchmark to measure multiversion dispatch overhead.
//!
//! Compares dispatch overhead between:
//! - Huge feature string (many features checked)
//! - Minimal feature string (few features checked)
//!
//! The goal is to determine if listing many features in the target string
//! adds measurable overhead to the runtime dispatch.

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

// ============================================================================
// Test functions with different target string sizes
// ============================================================================

/// Minimal target: just AVX-512F
#[multiversion::multiversion(targets("x86_64+avx512f"))]
fn sum_minimal(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Medium target: x86-64-v4 core (5 AVX-512 features)
#[multiversion::multiversion(targets("x86_64+avx512f+avx512bw+avx512dq+avx512vl+avx512cd"))]
fn sum_medium(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Huge target: full psABI v4 + extras (20+ features)
#[multiversion::multiversion(targets(
    "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt+cmpxchg16b+avx+avx2+bmi1+bmi2+f16c+fma+lzcnt+movbe+xsave+fxsr+avx512f+avx512bw+avx512dq+avx512vl+avx512cd+gfni+vaes+vpclmulqdq"
))]
fn sum_huge(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// Multiple targets: v4, v3, v2 (dispatch chooses best)
#[multiversion::multiversion(targets(
    "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt+cmpxchg16b+avx+avx2+bmi1+bmi2+f16c+fma+lzcnt+movbe+xsave+fxsr+avx512f+avx512bw+avx512dq+avx512vl+avx512cd+gfni+vaes+vpclmulqdq",
    "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt+cmpxchg16b+avx+avx2+bmi1+bmi2+f16c+fma+lzcnt+movbe+xsave+fxsr",
    "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt"
))]
fn sum_multi_target(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// 6 different targets - maximum dispatch complexity
#[multiversion::multiversion(targets(
    // v4 with all extensions
    "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt+cmpxchg16b+avx+avx2+bmi1+bmi2+f16c+fma+lzcnt+movbe+xsave+fxsr+avx512f+avx512bw+avx512dq+avx512vl+avx512cd+avx512vnni+avx512vbmi+avx512vbmi2+avx512bitalg+avx512vpopcntdq+avx512bf16+gfni+vaes+vpclmulqdq",
    // v4 core only
    "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt+cmpxchg16b+avx+avx2+bmi1+bmi2+f16c+fma+lzcnt+movbe+xsave+fxsr+avx512f+avx512bw+avx512dq+avx512vl+avx512cd",
    // v3 full
    "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt+cmpxchg16b+avx+avx2+bmi1+bmi2+f16c+fma+lzcnt+movbe+xsave+fxsr",
    // v3 minimal
    "x86_64+avx+avx2+fma",
    // v2
    "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt",
    // v1 (baseline)
    "x86_64+sse+sse2"
))]
fn sum_6_targets(data: &[f32]) -> f32 {
    data.iter().sum()
}

/// No multiversion - baseline for comparison
fn sum_baseline(data: &[f32]) -> f32 {
    data.iter().sum()
}

// ============================================================================
// Benchmarks
// ============================================================================

fn bench_dispatch_overhead(c: &mut Criterion) {
    // Small data to emphasize dispatch overhead vs actual work
    let small_data: Vec<f32> = (0..64).map(|i| i as f32).collect();
    // Medium data
    let medium_data: Vec<f32> = (0..1024).map(|i| i as f32).collect();
    // Large data where dispatch overhead should be negligible
    let large_data: Vec<f32> = (0..65536).map(|i| i as f32).collect();

    let mut group = c.benchmark_group("dispatch_overhead_small");
    group.throughput(Throughput::Elements(small_data.len() as u64));

    group.bench_function("baseline_no_multiversion", |b| {
        b.iter(|| sum_baseline(black_box(&small_data)))
    });
    group.bench_function("minimal_1_feature", |b| {
        b.iter(|| sum_minimal(black_box(&small_data)))
    });
    group.bench_function("medium_5_features", |b| {
        b.iter(|| sum_medium(black_box(&small_data)))
    });
    group.bench_function("huge_27_features", |b| {
        b.iter(|| sum_huge(black_box(&small_data)))
    });
    group.bench_function("multi_target_3_versions", |b| {
        b.iter(|| sum_multi_target(black_box(&small_data)))
    });
    group.bench_function("multi_target_6_versions", |b| {
        b.iter(|| sum_6_targets(black_box(&small_data)))
    });
    group.finish();

    let mut group = c.benchmark_group("dispatch_overhead_medium");
    group.throughput(Throughput::Elements(medium_data.len() as u64));

    group.bench_function("baseline_no_multiversion", |b| {
        b.iter(|| sum_baseline(black_box(&medium_data)))
    });
    group.bench_function("minimal_1_feature", |b| {
        b.iter(|| sum_minimal(black_box(&medium_data)))
    });
    group.bench_function("huge_27_features", |b| {
        b.iter(|| sum_huge(black_box(&medium_data)))
    });
    group.bench_function("multi_target_3_versions", |b| {
        b.iter(|| sum_multi_target(black_box(&medium_data)))
    });
    group.bench_function("multi_target_6_versions", |b| {
        b.iter(|| sum_6_targets(black_box(&medium_data)))
    });
    group.finish();

    let mut group = c.benchmark_group("dispatch_overhead_large");
    group.throughput(Throughput::Elements(large_data.len() as u64));

    group.bench_function("baseline_no_multiversion", |b| {
        b.iter(|| sum_baseline(black_box(&large_data)))
    });
    group.bench_function("minimal_1_feature", |b| {
        b.iter(|| sum_minimal(black_box(&large_data)))
    });
    group.bench_function("huge_27_features", |b| {
        b.iter(|| sum_huge(black_box(&large_data)))
    });
    group.bench_function("multi_target_3_versions", |b| {
        b.iter(|| sum_multi_target(black_box(&large_data)))
    });
    group.bench_function("multi_target_6_versions", |b| {
        b.iter(|| sum_6_targets(black_box(&large_data)))
    });
    group.finish();
}

/// Benchmark repeated calls to measure per-call dispatch overhead
fn bench_repeated_dispatch(c: &mut Criterion) {
    let data: Vec<f32> = (0..64).map(|i| i as f32).collect();

    let mut group = c.benchmark_group("repeated_dispatch");

    // Call the function many times in a loop to amortize any one-time costs
    group.bench_function("baseline_1000_calls", |b| {
        b.iter(|| {
            let mut total = 0.0f32;
            for _ in 0..1000 {
                total += sum_baseline(black_box(&data));
            }
            total
        })
    });

    group.bench_function("minimal_1000_calls", |b| {
        b.iter(|| {
            let mut total = 0.0f32;
            for _ in 0..1000 {
                total += sum_minimal(black_box(&data));
            }
            total
        })
    });

    group.bench_function("huge_1000_calls", |b| {
        b.iter(|| {
            let mut total = 0.0f32;
            for _ in 0..1000 {
                total += sum_huge(black_box(&data));
            }
            total
        })
    });

    group.bench_function("multi_target_3_1000_calls", |b| {
        b.iter(|| {
            let mut total = 0.0f32;
            for _ in 0..1000 {
                total += sum_multi_target(black_box(&data));
            }
            total
        })
    });

    group.bench_function("multi_target_6_1000_calls", |b| {
        b.iter(|| {
            let mut total = 0.0f32;
            for _ in 0..1000 {
                total += sum_6_targets(black_box(&data));
            }
            total
        })
    });

    group.finish();
}

criterion_group!(benches, bench_dispatch_overhead, bench_repeated_dispatch);
criterion_main!(benches);
