//! Runtime CPU feature detection tool
//!
//! Prints all detectable CPU features and their availability on the current system.
//! Also generates multiversion-compatible target strings for the current CPU.
//! Used by CI to document what features are available on GitHub Actions runners.

fn main() {
    println!("=== CPU Feature Detection Report ===");
    println!();
    println!("Target: {}", std::env::consts::ARCH);
    println!("OS: {}", std::env::consts::OS);
    println!();

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    detect_x86_features();

    #[cfg(target_arch = "aarch64")]
    detect_aarch64_features();

    #[cfg(target_arch = "wasm32")]
    detect_wasm_features();

    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "wasm32"
    )))]
    println!("No feature detection available for this architecture.");
}

// =============================================================================
// x86/x86_64
// =============================================================================

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86_features {
    /// All x86 features we want to detect, grouped by category
    pub const SSE_FEATURES: &[&str] =
        &["sse", "sse2", "sse3", "ssse3", "sse4.1", "sse4.2", "sse4a"];
    pub const AVX_FEATURES: &[&str] = &["avx", "avx2"];
    pub const AVX512_FEATURES: &[&str] = &[
        "avx512f",
        "avx512cd",
        "avx512bw",
        "avx512dq",
        "avx512vl",
        "avx512ifma",
        "avx512vbmi",
        "avx512vbmi2",
        "avx512vnni",
        "avx512bitalg",
        "avx512vpopcntdq",
        "avx512bf16",
        "avx512fp16",
    ];
    pub const SIMD_FEATURES: &[&str] = &["fma", "f16c", "avxvnni", "avxifma", "avxvnniint8"];
    pub const BIT_FEATURES: &[&str] = &["bmi1", "bmi2", "abm", "lzcnt", "popcnt", "tbm"];
    pub const CRYPTO_FEATURES: &[&str] = &["aes", "pclmulqdq", "sha", "vaes", "vpclmulqdq", "gfni"];
    pub const OTHER_FEATURES: &[&str] = &[
        "fxsr",
        "xsave",
        "cmpxchg16b",
        "movbe",
        "adx",
        "rtm",
        "rdrand",
        "rdseed",
    ];

    /// Features to include in multiversion target string (in order)
    pub const TARGET_STRING_FEATURES: &[&str] = &[
        // SSE family
        "sse",
        "sse2",
        "sse3",
        "ssse3",
        "sse4.1",
        "sse4.2", // Core
        "popcnt",
        "cmpxchg16b", // AVX family
        "avx",
        "avx2", // BMI
        "bmi1",
        "bmi2", // FMA and related
        "fma",
        "f16c",
        "lzcnt",
        "movbe", // Save/restore
        "fxsr",
        "xsave", // AVX-512
        "avx512f",
        "avx512bw",
        "avx512dq",
        "avx512vl",
        "avx512cd",
        "avx512ifma",
        "avx512vbmi",
        "avx512vbmi2",
        "avx512vnni",
        "avx512bitalg",
        "avx512vpopcntdq",
        "avx512bf16",
        // Crypto
        "aes",
        "pclmulqdq",
        "sha",
        "gfni",
        "vaes",
        "vpclmulqdq", // Other
        "adx",
        "rdrand",
        "rdseed",
    ];

    /// Macro to check a feature - required because is_x86_feature_detected! needs literals
    macro_rules! check {
        ($name:tt) => {
            std::arch::is_x86_feature_detected!($name)
        };
    }

    pub fn is_detected(name: &str) -> bool {
        match name {
            "sse" => check!("sse"),
            "sse2" => check!("sse2"),
            "sse3" => check!("sse3"),
            "ssse3" => check!("ssse3"),
            "sse4.1" => check!("sse4.1"),
            "sse4.2" => check!("sse4.2"),
            "sse4a" => check!("sse4a"),
            "avx" => check!("avx"),
            "avx2" => check!("avx2"),
            "avx512f" => check!("avx512f"),
            "avx512cd" => check!("avx512cd"),
            "avx512bw" => check!("avx512bw"),
            "avx512dq" => check!("avx512dq"),
            "avx512vl" => check!("avx512vl"),
            "avx512ifma" => check!("avx512ifma"),
            "avx512vbmi" => check!("avx512vbmi"),
            "avx512vbmi2" => check!("avx512vbmi2"),
            "avx512vnni" => check!("avx512vnni"),
            "avx512bitalg" => check!("avx512bitalg"),
            "avx512vpopcntdq" => check!("avx512vpopcntdq"),
            "avx512bf16" => check!("avx512bf16"),
            "avx512fp16" => check!("avx512fp16"),
            "fma" => check!("fma"),
            "f16c" => check!("f16c"),
            "avxvnni" => check!("avxvnni"),
            "avxifma" => check!("avxifma"),
            "avxvnniint8" => check!("avxvnniint8"),
            "bmi1" => check!("bmi1"),
            "bmi2" => check!("bmi2"),
            "abm" => check!("abm"),
            "lzcnt" => check!("lzcnt"),
            "popcnt" => check!("popcnt"),
            "tbm" => check!("tbm"),
            "aes" => check!("aes"),
            "pclmulqdq" => check!("pclmulqdq"),
            "sha" => check!("sha"),
            "vaes" => check!("vaes"),
            "vpclmulqdq" => check!("vpclmulqdq"),
            "gfni" => check!("gfni"),
            "fxsr" => check!("fxsr"),
            "xsave" => check!("xsave"),
            "cmpxchg16b" => check!("cmpxchg16b"),
            "movbe" => check!("movbe"),
            "adx" => check!("adx"),
            "rtm" => check!("rtm"),
            "rdrand" => check!("rdrand"),
            "rdseed" => check!("rdseed"),
            _ => false,
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn detect_x86_features() {
    use x86_features::*;

    println!("## x86/x86_64 Features");
    println!();

    fn print_features(title: &str, features: &[&str]) {
        println!("### {}", title);
        for &name in features {
            let detected = is_detected(name);
            println!("  {}: {}", name, if detected { "✓" } else { "✗" });
        }
        println!();
    }

    print_features("SSE Family", SSE_FEATURES);
    print_features("AVX Family", AVX_FEATURES);
    print_features("AVX-512 Family", AVX512_FEATURES);
    print_features("Other SIMD", SIMD_FEATURES);
    print_features("Bit Manipulation", BIT_FEATURES);
    print_features("Crypto", CRYPTO_FEATURES);
    print_features("Other", OTHER_FEATURES);

    // Microarch levels
    println!("### Microarchitecture Level Summary");
    let has_v2 = is_detected("sse4.2") && is_detected("popcnt");
    let has_v3 = has_v2
        && is_detected("avx2")
        && is_detected("fma")
        && is_detected("bmi1")
        && is_detected("bmi2");
    let has_v4 = has_v3
        && is_detected("avx512f")
        && is_detected("avx512bw")
        && is_detected("avx512dq")
        && is_detected("avx512vl")
        && is_detected("avx512cd");

    println!(
        "  x86-64-v2 (SSE4.2+POPCNT): {}",
        if has_v2 { "✓" } else { "✗" }
    );
    println!(
        "  x86-64-v3 (AVX2+FMA+BMI): {}",
        if has_v3 { "✓" } else { "✗" }
    );
    println!("  x86-64-v4 (AVX-512): {}", if has_v4 { "✓" } else { "✗" });
    println!();

    // Target string
    println!("### Multiversion Target String");
    println!();
    println!("  {}", compose_x86_target_string());
    println!();
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn compose_x86_target_string() -> String {
    use x86_features::*;

    let mut features = Vec::new();

    #[cfg(target_arch = "x86_64")]
    features.push("x86_64");
    #[cfg(target_arch = "x86")]
    features.push("x86");

    for &name in TARGET_STRING_FEATURES {
        if is_detected(name) {
            features.push(name);
        }
    }

    features.join("+")
}

// =============================================================================
// aarch64
// =============================================================================

#[cfg(target_arch = "aarch64")]
mod aarch64_features {
    pub const SIMD_FEATURES: &[&str] = &[
        "neon", "asimd", "fp16", "dotprod", "i8mm", "bf16", "fhm", "rdm", "fcma",
    ];
    pub const SVE_FEATURES: &[&str] = &[
        "sve",
        "sve2",
        "sve2-aes",
        "sve2-sm4",
        "sve2-sha3",
        "sve2-bitperm",
        "f32mm",
        "f64mm",
    ];
    pub const CRYPTO_FEATURES: &[&str] = &["aes", "sha2", "sha3", "sm4", "pmull"];
    pub const ATOMIC_FEATURES: &[&str] = &["lse", "lse2", "rcpc", "rcpc2", "crc"];
    pub const OTHER_FEATURES: &[&str] = &[
        "fp", "jsconv", "dpb", "dpb2", "frintts", "flagm", "ssbs", "sb", "paca", "pacg", "dit",
        "bti", "mte", "tme",
    ];

    pub const TARGET_STRING_FEATURES: &[&str] = &[
        "neon",
        "lse",
        "lse2",
        "aes",
        "sha2",
        "sha3",
        "sm4",
        "pmull",
        "crc",
        "dotprod",
        "fp16",
        "fhm",
        "rdm",
        "fcma",
        "i8mm",
        "bf16",
        "rcpc",
        "rcpc2",
        "sve",
        "sve2",
        "sve2-aes",
        "sve2-bitperm",
        "sve2-sha3",
        "sve2-sm4",
        "f32mm",
        "f64mm",
        "jsconv",
        "dpb",
        "dpb2",
        "frintts",
        "flagm",
        "sb",
        "paca",
        "pacg",
        "dit",
        "bti",
    ];

    macro_rules! check {
        ($name:tt) => {
            std::arch::is_aarch64_feature_detected!($name)
        };
    }

    pub fn is_detected(name: &str) -> bool {
        match name {
            "neon" => check!("neon"),
            "asimd" => check!("asimd"),
            "fp16" => check!("fp16"),
            "dotprod" => check!("dotprod"),
            "i8mm" => check!("i8mm"),
            "bf16" => check!("bf16"),
            "fhm" => check!("fhm"),
            "rdm" => check!("rdm"),
            "fcma" => check!("fcma"),
            "sve" => check!("sve"),
            "sve2" => check!("sve2"),
            "sve2-aes" => check!("sve2-aes"),
            "sve2-sm4" => check!("sve2-sm4"),
            "sve2-sha3" => check!("sve2-sha3"),
            "sve2-bitperm" => check!("sve2-bitperm"),
            "f32mm" => check!("f32mm"),
            "f64mm" => check!("f64mm"),
            "aes" => check!("aes"),
            "sha2" => check!("sha2"),
            "sha3" => check!("sha3"),
            "sm4" => check!("sm4"),
            "pmull" => check!("pmull"),
            "lse" => check!("lse"),
            "lse2" => check!("lse2"),
            "rcpc" => check!("rcpc"),
            "rcpc2" => check!("rcpc2"),
            "crc" => check!("crc"),
            "fp" => check!("fp"),
            "jsconv" => check!("jsconv"),
            "dpb" => check!("dpb"),
            "dpb2" => check!("dpb2"),
            "frintts" => check!("frintts"),
            "flagm" => check!("flagm"),
            "ssbs" => check!("ssbs"),
            "sb" => check!("sb"),
            "paca" => check!("paca"),
            "pacg" => check!("pacg"),
            "dit" => check!("dit"),
            "bti" => check!("bti"),
            "mte" => check!("mte"),
            "tme" => check!("tme"),
            _ => false,
        }
    }
}

#[cfg(target_arch = "aarch64")]
fn detect_aarch64_features() {
    use aarch64_features::*;

    println!("## aarch64 Features");
    println!();

    fn print_features(title: &str, features: &[&str]) {
        println!("### {}", title);
        for &name in features {
            let detected = is_detected(name);
            println!("  {}: {}", name, if detected { "✓" } else { "✗" });
        }
        println!();
    }

    print_features("SIMD", SIMD_FEATURES);
    print_features("SVE", SVE_FEATURES);
    print_features("Crypto", CRYPTO_FEATURES);
    print_features("Atomics & Memory", ATOMIC_FEATURES);
    print_features("Other", OTHER_FEATURES);

    // Preset levels
    println!("### Preset Level Summary");
    let has_dotprod = is_detected("dotprod") && is_detected("fp16");
    let has_apple_m1 = has_dotprod && is_detected("sha3") && is_detected("fcma");
    let has_sve2 = is_detected("sve2") && is_detected("i8mm") && is_detected("bf16");

    println!(
        "  aarch64-dotprod (dotprod+fp16): {}",
        if has_dotprod { "✓" } else { "✗" }
    );
    println!(
        "  aarch64-apple-m1 (sha3+fcma): {}",
        if has_apple_m1 { "✓" } else { "✗" }
    );
    println!(
        "  aarch64-sve2 (sve2+i8mm+bf16): {}",
        if has_sve2 { "✓" } else { "✗" }
    );
    println!();

    // Target string
    println!("### Multiversion Target String");
    println!();
    println!("  {}", compose_aarch64_target_string());
    println!();
}

#[cfg(target_arch = "aarch64")]
fn compose_aarch64_target_string() -> String {
    use aarch64_features::*;

    let mut features = vec!["aarch64"];

    for &name in TARGET_STRING_FEATURES {
        if is_detected(name) {
            features.push(name);
        }
    }

    features.join("+")
}

// =============================================================================
// wasm32
// =============================================================================

#[cfg(target_arch = "wasm32")]
fn detect_wasm_features() {
    println!("## wasm32 Features");
    println!();
    println!("Note: WebAssembly has no runtime feature detection.");
    println!("Features are determined at compile time via -C target-feature.");
    println!();

    println!("### Compile-time Features");

    #[cfg(target_feature = "simd128")]
    println!("  simd128: ✓ (compiled with)");
    #[cfg(not(target_feature = "simd128"))]
    println!("  simd128: ✗ (not compiled with)");

    #[cfg(target_feature = "relaxed-simd")]
    println!("  relaxed-simd: ✓ (compiled with)");
    #[cfg(not(target_feature = "relaxed-simd"))]
    println!("  relaxed-simd: ✗ (not compiled with)");

    #[cfg(target_feature = "atomics")]
    println!("  atomics: ✓ (compiled with)");
    #[cfg(not(target_feature = "atomics"))]
    println!("  atomics: ✗ (not compiled with)");

    #[cfg(target_feature = "bulk-memory")]
    println!("  bulk-memory: ✓ (compiled with)");
    #[cfg(not(target_feature = "bulk-memory"))]
    println!("  bulk-memory: ✗ (not compiled with)");

    println!();

    println!("### Multiversion Target String (compile-time)");
    println!();

    let mut features = vec!["wasm32"];

    #[cfg(target_feature = "simd128")]
    features.push("simd128");
    #[cfg(target_feature = "relaxed-simd")]
    features.push("relaxed-simd");
    #[cfg(target_feature = "atomics")]
    features.push("atomics");
    #[cfg(target_feature = "bulk-memory")]
    features.push("bulk-memory");

    println!("  {}", features.join("+"));
    println!();
}
