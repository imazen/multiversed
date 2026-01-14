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

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn detect_x86_features() {
    use std::arch::is_x86_feature_detected;

    println!("## x86/x86_64 Features");
    println!();

    // Macro to check and print feature status
    macro_rules! check_feature {
        ($name:tt) => {
            let detected = is_x86_feature_detected!($name);
            println!("  {}: {}", $name, if detected { "✓" } else { "✗" });
        };
    }

    println!("### SSE Family");
    check_feature!("sse");
    check_feature!("sse2");
    check_feature!("sse3");
    check_feature!("ssse3");
    check_feature!("sse4.1");
    check_feature!("sse4.2");
    check_feature!("sse4a");
    println!();

    println!("### AVX Family");
    check_feature!("avx");
    check_feature!("avx2");
    println!();

    println!("### AVX-512 Family");
    check_feature!("avx512f");
    check_feature!("avx512cd");
    check_feature!("avx512bw");
    check_feature!("avx512dq");
    check_feature!("avx512vl");
    check_feature!("avx512ifma");
    check_feature!("avx512vbmi");
    check_feature!("avx512vbmi2");
    check_feature!("avx512vnni");
    check_feature!("avx512bitalg");
    check_feature!("avx512vpopcntdq");
    check_feature!("avx512bf16");
    check_feature!("avx512fp16");
    println!();

    println!("### Other SIMD");
    check_feature!("fma");
    check_feature!("f16c");
    check_feature!("avxvnni");
    check_feature!("avxifma");
    check_feature!("avxvnniint8");
    println!();

    println!("### Bit Manipulation");
    check_feature!("bmi1");
    check_feature!("bmi2");
    check_feature!("abm");
    check_feature!("lzcnt");
    check_feature!("popcnt");
    check_feature!("tbm");
    println!();

    println!("### Crypto");
    check_feature!("aes");
    check_feature!("pclmulqdq");
    check_feature!("sha");
    check_feature!("vaes");
    check_feature!("vpclmulqdq");
    check_feature!("gfni");
    println!();

    println!("### Other");
    check_feature!("fxsr");
    check_feature!("xsave");
    check_feature!("cmpxchg16b");
    check_feature!("movbe");
    check_feature!("adx");
    check_feature!("rtm");
    check_feature!("rdrand");
    check_feature!("rdseed");
    println!();

    // Print summary of microarch levels
    println!("### Microarchitecture Level Summary");
    let has_v2 = is_x86_feature_detected!("sse4.2") && is_x86_feature_detected!("popcnt");
    let has_v3 = has_v2
        && is_x86_feature_detected!("avx2")
        && is_x86_feature_detected!("fma")
        && is_x86_feature_detected!("bmi1")
        && is_x86_feature_detected!("bmi2");
    let has_v4 = has_v3
        && is_x86_feature_detected!("avx512f")
        && is_x86_feature_detected!("avx512bw")
        && is_x86_feature_detected!("avx512dq")
        && is_x86_feature_detected!("avx512vl")
        && is_x86_feature_detected!("avx512cd");

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

    // Generate multiversion target string
    println!("### Multiversion Target String");
    let target_string = compose_x86_target_string();
    println!();
    println!("  {}", target_string);
    println!();
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn compose_x86_target_string() -> String {
    use std::arch::is_x86_feature_detected;

    let mut features = Vec::new();

    // Base architecture
    #[cfg(target_arch = "x86_64")]
    features.push("x86_64");
    #[cfg(target_arch = "x86")]
    features.push("x86");

    // Collect all detected features in a sensible order
    // SSE family
    if is_x86_feature_detected!("sse") {
        features.push("sse");
    }
    if is_x86_feature_detected!("sse2") {
        features.push("sse2");
    }
    if is_x86_feature_detected!("sse3") {
        features.push("sse3");
    }
    if is_x86_feature_detected!("ssse3") {
        features.push("ssse3");
    }
    if is_x86_feature_detected!("sse4.1") {
        features.push("sse4.1");
    }
    if is_x86_feature_detected!("sse4.2") {
        features.push("sse4.2");
    }

    // Core features
    if is_x86_feature_detected!("popcnt") {
        features.push("popcnt");
    }
    if is_x86_feature_detected!("cmpxchg16b") {
        features.push("cmpxchg16b");
    }

    // AVX family
    if is_x86_feature_detected!("avx") {
        features.push("avx");
    }
    if is_x86_feature_detected!("avx2") {
        features.push("avx2");
    }

    // BMI
    if is_x86_feature_detected!("bmi1") {
        features.push("bmi1");
    }
    if is_x86_feature_detected!("bmi2") {
        features.push("bmi2");
    }

    // FMA and related
    if is_x86_feature_detected!("fma") {
        features.push("fma");
    }
    if is_x86_feature_detected!("f16c") {
        features.push("f16c");
    }
    if is_x86_feature_detected!("lzcnt") {
        features.push("lzcnt");
    }
    if is_x86_feature_detected!("movbe") {
        features.push("movbe");
    }

    // Save/restore
    if is_x86_feature_detected!("fxsr") {
        features.push("fxsr");
    }
    if is_x86_feature_detected!("xsave") {
        features.push("xsave");
    }

    // AVX-512 family
    if is_x86_feature_detected!("avx512f") {
        features.push("avx512f");
    }
    if is_x86_feature_detected!("avx512bw") {
        features.push("avx512bw");
    }
    if is_x86_feature_detected!("avx512dq") {
        features.push("avx512dq");
    }
    if is_x86_feature_detected!("avx512vl") {
        features.push("avx512vl");
    }
    if is_x86_feature_detected!("avx512cd") {
        features.push("avx512cd");
    }
    if is_x86_feature_detected!("avx512ifma") {
        features.push("avx512ifma");
    }
    if is_x86_feature_detected!("avx512vbmi") {
        features.push("avx512vbmi");
    }
    if is_x86_feature_detected!("avx512vbmi2") {
        features.push("avx512vbmi2");
    }
    if is_x86_feature_detected!("avx512vnni") {
        features.push("avx512vnni");
    }
    if is_x86_feature_detected!("avx512bitalg") {
        features.push("avx512bitalg");
    }
    if is_x86_feature_detected!("avx512vpopcntdq") {
        features.push("avx512vpopcntdq");
    }
    if is_x86_feature_detected!("avx512bf16") {
        features.push("avx512bf16");
    }

    // Crypto
    if is_x86_feature_detected!("aes") {
        features.push("aes");
    }
    if is_x86_feature_detected!("pclmulqdq") {
        features.push("pclmulqdq");
    }
    if is_x86_feature_detected!("sha") {
        features.push("sha");
    }
    if is_x86_feature_detected!("gfni") {
        features.push("gfni");
    }
    if is_x86_feature_detected!("vaes") {
        features.push("vaes");
    }
    if is_x86_feature_detected!("vpclmulqdq") {
        features.push("vpclmulqdq");
    }

    // Other useful features
    if is_x86_feature_detected!("adx") {
        features.push("adx");
    }
    if is_x86_feature_detected!("rdrand") {
        features.push("rdrand");
    }
    if is_x86_feature_detected!("rdseed") {
        features.push("rdseed");
    }

    features.join("+")
}

#[cfg(target_arch = "aarch64")]
fn detect_aarch64_features() {
    use std::arch::is_aarch64_feature_detected;

    println!("## aarch64 Features");
    println!();

    // Use std_detect crate macros
    macro_rules! check_feature {
        ($name:tt) => {
            let detected = is_aarch64_feature_detected!($name);
            println!("  {}: {}", $name, if detected { "✓" } else { "✗" });
        };
    }

    println!("### SIMD");
    check_feature!("neon");
    check_feature!("asimd");
    check_feature!("fp16");
    check_feature!("dotprod");
    check_feature!("i8mm");
    check_feature!("bf16");
    check_feature!("fhm");
    check_feature!("rdm");
    check_feature!("fcma");
    println!();

    println!("### SVE");
    check_feature!("sve");
    check_feature!("sve2");
    check_feature!("sve2-aes");
    check_feature!("sve2-sm4");
    check_feature!("sve2-sha3");
    check_feature!("sve2-bitperm");
    check_feature!("f32mm");
    check_feature!("f64mm");
    println!();

    println!("### Crypto");
    check_feature!("aes");
    check_feature!("sha2");
    check_feature!("sha3");
    check_feature!("sm4");
    check_feature!("pmull");
    println!();

    println!("### Atomics & Memory");
    check_feature!("lse");
    check_feature!("lse2");
    check_feature!("rcpc");
    check_feature!("rcpc2");
    check_feature!("crc");
    println!();

    println!("### Other");
    check_feature!("fp");
    check_feature!("jsconv");
    check_feature!("dpb");
    check_feature!("dpb2");
    check_feature!("frintts");
    check_feature!("flagm");
    check_feature!("ssbs");
    check_feature!("sb");
    check_feature!("paca");
    check_feature!("pacg");
    check_feature!("dit");
    check_feature!("bti");
    check_feature!("mte");
    check_feature!("tme");
    println!();

    // Print summary of preset levels
    println!("### Preset Level Summary");
    let has_dotprod =
        is_aarch64_feature_detected!("dotprod") && is_aarch64_feature_detected!("fp16");
    let has_apple_m1 =
        has_dotprod && is_aarch64_feature_detected!("sha3") && is_aarch64_feature_detected!("fcma");
    let has_sve2 = is_aarch64_feature_detected!("sve2")
        && is_aarch64_feature_detected!("i8mm")
        && is_aarch64_feature_detected!("bf16");

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

    // Generate multiversion target string
    println!("### Multiversion Target String");
    let target_string = compose_aarch64_target_string();
    println!();
    println!("  {}", target_string);
    println!();
}

#[cfg(target_arch = "aarch64")]
fn compose_aarch64_target_string() -> String {
    use std::arch::is_aarch64_feature_detected;

    let mut features = Vec::new();

    // Base architecture
    features.push("aarch64");

    // Core SIMD (neon is baseline, always present)
    if is_aarch64_feature_detected!("neon") {
        features.push("neon");
    }

    // Atomics
    if is_aarch64_feature_detected!("lse") {
        features.push("lse");
    }
    if is_aarch64_feature_detected!("lse2") {
        features.push("lse2");
    }

    // Crypto
    if is_aarch64_feature_detected!("aes") {
        features.push("aes");
    }
    if is_aarch64_feature_detected!("sha2") {
        features.push("sha2");
    }
    if is_aarch64_feature_detected!("sha3") {
        features.push("sha3");
    }
    if is_aarch64_feature_detected!("sm4") {
        features.push("sm4");
    }
    if is_aarch64_feature_detected!("pmull") {
        features.push("pmull");
    }

    // CRC
    if is_aarch64_feature_detected!("crc") {
        features.push("crc");
    }

    // SIMD extensions
    if is_aarch64_feature_detected!("dotprod") {
        features.push("dotprod");
    }
    if is_aarch64_feature_detected!("fp16") {
        features.push("fp16");
    }
    if is_aarch64_feature_detected!("fhm") {
        features.push("fhm");
    }
    if is_aarch64_feature_detected!("rdm") {
        features.push("rdm");
    }
    if is_aarch64_feature_detected!("fcma") {
        features.push("fcma");
    }
    if is_aarch64_feature_detected!("i8mm") {
        features.push("i8mm");
    }
    if is_aarch64_feature_detected!("bf16") {
        features.push("bf16");
    }

    // RCPC
    if is_aarch64_feature_detected!("rcpc") {
        features.push("rcpc");
    }
    if is_aarch64_feature_detected!("rcpc2") {
        features.push("rcpc2");
    }

    // SVE family
    if is_aarch64_feature_detected!("sve") {
        features.push("sve");
    }
    if is_aarch64_feature_detected!("sve2") {
        features.push("sve2");
    }
    if is_aarch64_feature_detected!("sve2-aes") {
        features.push("sve2-aes");
    }
    if is_aarch64_feature_detected!("sve2-bitperm") {
        features.push("sve2-bitperm");
    }
    if is_aarch64_feature_detected!("sve2-sha3") {
        features.push("sve2-sha3");
    }
    if is_aarch64_feature_detected!("sve2-sm4") {
        features.push("sve2-sm4");
    }
    if is_aarch64_feature_detected!("f32mm") {
        features.push("f32mm");
    }
    if is_aarch64_feature_detected!("f64mm") {
        features.push("f64mm");
    }

    // Other features
    if is_aarch64_feature_detected!("jsconv") {
        features.push("jsconv");
    }
    if is_aarch64_feature_detected!("dpb") {
        features.push("dpb");
    }
    if is_aarch64_feature_detected!("dpb2") {
        features.push("dpb2");
    }
    if is_aarch64_feature_detected!("frintts") {
        features.push("frintts");
    }
    if is_aarch64_feature_detected!("flagm") {
        features.push("flagm");
    }
    if is_aarch64_feature_detected!("sb") {
        features.push("sb");
    }
    if is_aarch64_feature_detected!("paca") {
        features.push("paca");
    }
    if is_aarch64_feature_detected!("pacg") {
        features.push("pacg");
    }
    if is_aarch64_feature_detected!("dit") {
        features.push("dit");
    }
    if is_aarch64_feature_detected!("bti") {
        features.push("bti");
    }

    features.join("+")
}

#[cfg(target_arch = "wasm32")]
fn detect_wasm_features() {
    println!("## wasm32 Features");
    println!();
    println!("Note: WebAssembly has no runtime feature detection.");
    println!("Features are determined at compile time via -C target-feature.");
    println!();

    // Compile-time detection only
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

    // Generate compile-time target string
    println!("### Multiversion Target String (compile-time)");
    let target_string = compose_wasm_target_string();
    println!();
    println!("  {}", target_string);
    println!();
}

#[cfg(target_arch = "wasm32")]
fn compose_wasm_target_string() -> String {
    let mut features = Vec::new();

    features.push("wasm32");

    #[cfg(target_feature = "simd128")]
    features.push("simd128");

    #[cfg(target_feature = "relaxed-simd")]
    features.push("relaxed-simd");

    #[cfg(target_feature = "atomics")]
    features.push("atomics");

    #[cfg(target_feature = "bulk-memory")]
    features.push("bulk-memory");

    features.join("+")
}
