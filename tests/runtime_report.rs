//! Runtime feature detection report.
//!
//! Reports which multiversed presets are available on the current CPU.
//! Always passes — this is diagnostic, not a correctness check.

#[test]
fn report_preset_availability() {
    println!();
    println!("=== multiversed preset availability ===");
    println!();

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    report_x86();

    #[cfg(target_arch = "aarch64")]
    report_aarch64();

    report_wasm();

    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "wasm32"
    )))]
    println!("  (no preset detection for this architecture)");

    println!();
}

// ============================================================================
// x86/x86_64
// ============================================================================

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn report_x86() {
    use std::arch::is_x86_feature_detected;

    macro_rules! check {
        ($feat:tt) => {
            is_x86_feature_detected!($feat)
        };
    }

    // x86-64-v2: X64V2Token
    let v2_features: &[(&str, bool)] = &[
        ("sse3", check!("sse3")),
        ("ssse3", check!("ssse3")),
        ("sse4.1", check!("sse4.1")),
        ("sse4.2", check!("sse4.2")),
        ("popcnt", check!("popcnt")),
        ("cmpxchg16b", check!("cmpxchg16b")),
    ];

    // x86-64-v3: X64V3Token (delta over v2)
    let v3_features: &[(&str, bool)] = &[
        ("avx", check!("avx")),
        ("avx2", check!("avx2")),
        ("fma", check!("fma")),
        ("bmi1", check!("bmi1")),
        ("bmi2", check!("bmi2")),
        ("f16c", check!("f16c")),
        ("lzcnt", check!("lzcnt")),
        ("movbe", check!("movbe")),
    ];

    // x86-64-v4: X64V4Token (delta over v3)
    let v4_features: &[(&str, bool)] = &[
        ("avx512f", check!("avx512f")),
        ("avx512bw", check!("avx512bw")),
        ("avx512cd", check!("avx512cd")),
        ("avx512dq", check!("avx512dq")),
        ("avx512vl", check!("avx512vl")),
    ];

    // x86-64-v4x: X64V4xToken (delta over v4)
    let v4x_features: &[(&str, bool)] = &[
        ("avx512vpopcntdq", check!("avx512vpopcntdq")),
        ("avx512ifma", check!("avx512ifma")),
        ("avx512vbmi", check!("avx512vbmi")),
        ("avx512vbmi2", check!("avx512vbmi2")),
        ("avx512bitalg", check!("avx512bitalg")),
        ("avx512vnni", check!("avx512vnni")),
        ("vpclmulqdq", check!("vpclmulqdq")),
        ("gfni", check!("gfni")),
        ("vaes", check!("vaes")),
    ];

    let has_v2 = v2_features.iter().all(|(_, ok)| *ok);
    let has_v3 = has_v2 && v3_features.iter().all(|(_, ok)| *ok);
    let has_v4 = has_v3 && v4_features.iter().all(|(_, ok)| *ok);
    let has_v4x = has_v4 && v4x_features.iter().all(|(_, ok)| *ok);

    print_preset("x86-64-v2", "X64V2Token", has_v2, v2_features);
    print_preset("x86-64-v3", "X64V3Token", has_v3, v3_features);
    print_preset("x86-64-v4", "X64V4Token", has_v4, v4_features);
    print_preset("x86-64-v4x", "X64V4xToken", has_v4x, v4x_features);
}

// ============================================================================
// aarch64
// ============================================================================

#[cfg(target_arch = "aarch64")]
fn report_aarch64() {
    use std::arch::is_aarch64_feature_detected;

    macro_rules! check {
        ($feat:tt) => {
            is_aarch64_feature_detected!($feat)
        };
    }

    // arm64 / arm64-v2: Arm64V2Token ("arm64" is an alias for "arm64-v2")
    let v2_features: &[(&str, bool)] = &[
        ("neon", check!("neon")),
        ("crc", check!("crc")),
        ("rdm", check!("rdm")),
        ("dotprod", check!("dotprod")),
        ("fp16", check!("fp16")),
        ("aes", check!("aes")),
        ("sha2", check!("sha2")),
    ];

    // arm64-v3: Arm64V3Token (delta over v2)
    let v3_features: &[(&str, bool)] = &[
        ("fhm", check!("fhm")),
        ("fcma", check!("fcma")),
        ("sha3", check!("sha3")),
        ("i8mm", check!("i8mm")),
        ("bf16", check!("bf16")),
    ];

    let has_v2 = v2_features.iter().all(|(_, ok)| *ok);
    let has_v3 = has_v2 && v3_features.iter().all(|(_, ok)| *ok);

    print_preset("arm64/arm64-v2", "Arm64V2Token", has_v2, v2_features);
    print_preset("arm64-v3", "Arm64V3Token", has_v3, v3_features);
}

// ============================================================================
// wasm32
// ============================================================================

fn report_wasm() {
    // wasm32 has no runtime detection — check compile-time target_feature
    let has_simd128 = cfg!(target_feature = "simd128");
    let is_wasm = cfg!(target_arch = "wasm32");

    let status = if is_wasm {
        if has_simd128 { "YES" } else { "no" }
    } else {
        "n/a (not wasm32)"
    };

    println!("  wasm32-simd128  Wasm128Token    {status}",);
}

// ============================================================================
// Helpers
// ============================================================================

fn print_preset(name: &str, token: &str, available: bool, features: &[(&str, bool)]) {
    let status = if available { "YES" } else { "no" };
    println!("  {name:<16}  {token:<14}  {status}");

    if !available {
        let missing: Vec<&str> = features
            .iter()
            .filter(|(_, ok)| !ok)
            .map(|(name, _)| *name)
            .collect();
        if !missing.is_empty() {
            println!("    missing: {}", missing.join(", "));
        }
    }
}
