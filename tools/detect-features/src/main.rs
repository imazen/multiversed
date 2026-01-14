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
// x86/x86_64 - All features from is_x86_feature_detected! documentation
// =============================================================================

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86_features {
    // Grouped by category for display
    pub const SSE_FEATURES: &[&str] = &[
        "mmx", "sse", "sse2", "sse3", "ssse3", "sse4.1", "sse4.2", "sse4a",
    ];
    pub const AVX_FEATURES: &[&str] = &["avx", "avx2"];
    pub const AVX512_FEATURES: &[&str] = &[
        "avx512f",
        "avx512cd",
        "avx512er",
        "avx512pf",
        "avx512bw",
        "avx512dq",
        "avx512vl",
        "avx512ifma",
        "avx512vbmi",
        "avx512vbmi2",
        "avx512vpopcntdq",
        "avx512vnni",
        "avx512bitalg",
        "avx512bf16",
        "avx512vp2intersect",
        "avx512fp16",
    ];
    pub const AVX_EXT_FEATURES: &[&str] = &[
        "avxvnni",
        "avxvnniint8",
        "avxvnniint16",
        "avxifma",
        "avxneconvert",
    ];
    // Note: AVX10 and AMX require nightly - omitted from stable builds
    pub const SIMD_FEATURES: &[&str] = &["fma", "f16c"];
    pub const BIT_FEATURES: &[&str] = &["bmi1", "bmi2", "abm", "lzcnt", "popcnt", "tbm"];
    pub const CRYPTO_FEATURES: &[&str] = &[
        "aes",
        "pclmulqdq",
        "sha",
        "sha512",
        "vaes",
        "vpclmulqdq",
        "gfni",
        "sm3",
        "sm4",
        "kl",
        "widekl",
    ];
    pub const STATE_FEATURES: &[&str] = &["fxsr", "xsave", "xsaveopt", "xsaves", "xsavec"];
    pub const OTHER_FEATURES: &[&str] = &[
        "cmpxchg16b",
        "movbe",
        "adx",
        "rtm",
        "rdrand",
        "rdseed",
        "tsc",
        "ermsb",
    ];

    /// Features to include in multiversion target string (most useful for SIMD)
    pub const TARGET_STRING_FEATURES: &[&str] = &[
        "sse",
        "sse2",
        "sse3",
        "ssse3",
        "sse4.1",
        "sse4.2",
        "popcnt",
        "cmpxchg16b",
        "avx",
        "avx2",
        "bmi1",
        "bmi2",
        "fma",
        "f16c",
        "lzcnt",
        "movbe",
        "fxsr",
        "xsave",
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
        "aes",
        "pclmulqdq",
        "sha",
        "gfni",
        "vaes",
        "vpclmulqdq",
        "adx",
        "rdrand",
        "rdseed",
    ];

    macro_rules! check {
        ($name:tt) => {
            std::arch::is_x86_feature_detected!($name)
        };
    }

    pub fn is_detected(name: &str) -> bool {
        match name {
            // SSE family
            "mmx" => check!("mmx"),
            "sse" => check!("sse"),
            "sse2" => check!("sse2"),
            "sse3" => check!("sse3"),
            "ssse3" => check!("ssse3"),
            "sse4.1" => check!("sse4.1"),
            "sse4.2" => check!("sse4.2"),
            "sse4a" => check!("sse4a"),
            // AVX family
            "avx" => check!("avx"),
            "avx2" => check!("avx2"),
            // AVX-512
            "avx512f" => check!("avx512f"),
            "avx512cd" => check!("avx512cd"),
            "avx512er" => check!("avx512er"),
            "avx512pf" => check!("avx512pf"),
            "avx512bw" => check!("avx512bw"),
            "avx512dq" => check!("avx512dq"),
            "avx512vl" => check!("avx512vl"),
            "avx512ifma" => check!("avx512ifma"),
            "avx512vbmi" => check!("avx512vbmi"),
            "avx512vbmi2" => check!("avx512vbmi2"),
            "avx512vpopcntdq" => check!("avx512vpopcntdq"),
            "avx512vnni" => check!("avx512vnni"),
            "avx512bitalg" => check!("avx512bitalg"),
            "avx512bf16" => check!("avx512bf16"),
            "avx512vp2intersect" => check!("avx512vp2intersect"),
            "avx512fp16" => check!("avx512fp16"),
            // AVX extensions
            "avxvnni" => check!("avxvnni"),
            "avxvnniint8" => check!("avxvnniint8"),
            "avxvnniint16" => check!("avxvnniint16"),
            "avxifma" => check!("avxifma"),
            "avxneconvert" => check!("avxneconvert"),
            // SIMD
            "fma" => check!("fma"),
            "f16c" => check!("f16c"),
            // Bit manipulation
            "bmi1" => check!("bmi1"),
            "bmi2" => check!("bmi2"),
            "abm" => check!("abm"),
            "lzcnt" => check!("lzcnt"),
            "popcnt" => check!("popcnt"),
            "tbm" => check!("tbm"),
            // Crypto
            "aes" => check!("aes"),
            "pclmulqdq" => check!("pclmulqdq"),
            "sha" => check!("sha"),
            "sha512" => check!("sha512"),
            "vaes" => check!("vaes"),
            "vpclmulqdq" => check!("vpclmulqdq"),
            "gfni" => check!("gfni"),
            "sm3" => check!("sm3"),
            "sm4" => check!("sm4"),
            "kl" => check!("kl"),
            "widekl" => check!("widekl"),
            // State
            "fxsr" => check!("fxsr"),
            "xsave" => check!("xsave"),
            "xsaveopt" => check!("xsaveopt"),
            "xsaves" => check!("xsaves"),
            "xsavec" => check!("xsavec"),
            // Other
            "cmpxchg16b" => check!("cmpxchg16b"),
            "movbe" => check!("movbe"),
            "adx" => check!("adx"),
            "rtm" => check!("rtm"),
            "rdrand" => check!("rdrand"),
            "rdseed" => check!("rdseed"),
            "tsc" => check!("tsc"),
            "ermsb" => check!("ermsb"),
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
    print_features("AVX Extensions", AVX_EXT_FEATURES);
    print_features("Other SIMD", SIMD_FEATURES);
    print_features("Bit Manipulation", BIT_FEATURES);
    print_features("Crypto", CRYPTO_FEATURES);
    print_features("State Management", STATE_FEATURES);
    print_features("Other", OTHER_FEATURES);
    // Note: AVX10 and AMX features require nightly Rust - omitted from stable builds

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
// aarch64 - All features from is_aarch64_feature_detected! documentation
// =============================================================================

#[cfg(target_arch = "aarch64")]
mod aarch64_features {
    pub const SIMD_FEATURES: &[&str] = &[
        "neon", "asimd", "fp", "fp16", "dotprod", "i8mm", "bf16", "fhm", "rdm", "fcma", "frintts",
    ];
    pub const FP8_FEATURES: &[&str] = &["fp8", "fp8dot2", "fp8dot4", "fp8fma", "fpmr"];
    pub const SVE_FEATURES: &[&str] = &[
        "sve",
        "sve2",
        "sve2p1",
        "sve-b16b16",
        "sve2-aes",
        "sve2-sm4",
        "sve2-sha3",
        "sve2-bitperm",
        "f32mm",
        "f64mm",
    ];
    pub const SME_FEATURES: &[&str] = &[
        "sme",
        "sme2",
        "sme2p1",
        "sme-b16b16",
        "sme-f16f16",
        "sme-f64f64",
        "sme-f8f16",
        "sme-f8f32",
        "sme-fa64",
        "sme-i16i64",
        "sme-lutv2",
    ];
    pub const SSVE_FEATURES: &[&str] = &["ssve-fp8dot2", "ssve-fp8dot4", "ssve-fp8fma"];
    pub const CRYPTO_FEATURES: &[&str] = &["aes", "sha2", "sha3", "sm4", "pmull"];
    pub const ATOMIC_FEATURES: &[&str] = &["lse", "lse2", "lse128", "rcpc", "rcpc2", "rcpc3"];
    pub const MEMORY_FEATURES: &[&str] = &["crc", "dpb", "dpb2", "mops", "mte"];
    pub const SECURITY_FEATURES: &[&str] =
        &["bti", "paca", "pacg", "pauth-lr", "sb", "ssbs", "dit"];
    pub const OTHER_FEATURES: &[&str] = &[
        "jsconv", "flagm", "flagm2", "ecv", "hbc", "lut", "rand", "tme", "wfxt", "ras", "cssc",
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
            // SIMD
            "neon" => check!("neon"),
            "asimd" => check!("asimd"),
            "fp" => check!("fp"),
            "fp16" => check!("fp16"),
            "dotprod" => check!("dotprod"),
            "i8mm" => check!("i8mm"),
            "bf16" => check!("bf16"),
            "fhm" => check!("fhm"),
            "rdm" => check!("rdm"),
            "fcma" => check!("fcma"),
            "frintts" => check!("frintts"),
            // FP8
            "fp8" => check!("fp8"),
            "fp8dot2" => check!("fp8dot2"),
            "fp8dot4" => check!("fp8dot4"),
            "fp8fma" => check!("fp8fma"),
            "fpmr" => check!("fpmr"),
            // SVE
            "sve" => check!("sve"),
            "sve2" => check!("sve2"),
            "sve2p1" => check!("sve2p1"),
            "sve-b16b16" => check!("sve-b16b16"),
            "sve2-aes" => check!("sve2-aes"),
            "sve2-sm4" => check!("sve2-sm4"),
            "sve2-sha3" => check!("sve2-sha3"),
            "sve2-bitperm" => check!("sve2-bitperm"),
            "f32mm" => check!("f32mm"),
            "f64mm" => check!("f64mm"),
            // SME
            "sme" => check!("sme"),
            "sme2" => check!("sme2"),
            "sme2p1" => check!("sme2p1"),
            "sme-b16b16" => check!("sme-b16b16"),
            "sme-f16f16" => check!("sme-f16f16"),
            "sme-f64f64" => check!("sme-f64f64"),
            "sme-f8f16" => check!("sme-f8f16"),
            "sme-f8f32" => check!("sme-f8f32"),
            "sme-fa64" => check!("sme-fa64"),
            "sme-i16i64" => check!("sme-i16i64"),
            "sme-lutv2" => check!("sme-lutv2"),
            // SSVE
            "ssve-fp8dot2" => check!("ssve-fp8dot2"),
            "ssve-fp8dot4" => check!("ssve-fp8dot4"),
            "ssve-fp8fma" => check!("ssve-fp8fma"),
            // Crypto
            "aes" => check!("aes"),
            "sha2" => check!("sha2"),
            "sha3" => check!("sha3"),
            "sm4" => check!("sm4"),
            "pmull" => check!("pmull"),
            // Atomics
            "lse" => check!("lse"),
            "lse2" => check!("lse2"),
            "lse128" => check!("lse128"),
            "rcpc" => check!("rcpc"),
            "rcpc2" => check!("rcpc2"),
            "rcpc3" => check!("rcpc3"),
            // Memory
            "crc" => check!("crc"),
            "dpb" => check!("dpb"),
            "dpb2" => check!("dpb2"),
            "mops" => check!("mops"),
            "mte" => check!("mte"),
            // Security
            "bti" => check!("bti"),
            "paca" => check!("paca"),
            "pacg" => check!("pacg"),
            "pauth-lr" => check!("pauth-lr"),
            "sb" => check!("sb"),
            "ssbs" => check!("ssbs"),
            "dit" => check!("dit"),
            // Other
            "jsconv" => check!("jsconv"),
            "flagm" => check!("flagm"),
            "flagm2" => check!("flagm2"),
            "ecv" => check!("ecv"),
            "hbc" => check!("hbc"),
            "lut" => check!("lut"),
            "rand" => check!("rand"),
            "tme" => check!("tme"),
            "wfxt" => check!("wfxt"),
            "ras" => check!("ras"),
            "cssc" => check!("cssc"),
            // Faminmax (missed earlier)
            "faminmax" => check!("faminmax"),
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
    print_features("FP8", FP8_FEATURES);
    print_features("SVE", SVE_FEATURES);
    print_features("SME (Matrix)", SME_FEATURES);
    print_features("SSVE", SSVE_FEATURES);
    print_features("Crypto", CRYPTO_FEATURES);
    print_features("Atomics", ATOMIC_FEATURES);
    print_features("Memory", MEMORY_FEATURES);
    print_features("Security", SECURITY_FEATURES);
    print_features("Other", OTHER_FEATURES);

    // Preset levels
    println!("### Preset Level Summary");
    let has_basic = is_detected("dotprod") && is_detected("fp16");
    let has_v84 = has_basic && is_detected("sha3") && is_detected("fcma");
    let has_sve = has_v84 && is_detected("sve") && is_detected("i8mm") && is_detected("bf16");
    let has_sve2 = has_sve && is_detected("sve2");

    println!(
        "  aarch64-basic (dotprod+fp16): {}",
        if has_basic { "✓" } else { "✗" }
    );
    println!(
        "  aarch64-v84 (sha3+fcma): {}",
        if has_v84 { "✓" } else { "✗" }
    );
    println!(
        "  aarch64-sve (sve+i8mm+bf16): {}",
        if has_sve { "✓" } else { "✗" }
    );
    println!(
        "  aarch64-sve2 (sve2): {}",
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
