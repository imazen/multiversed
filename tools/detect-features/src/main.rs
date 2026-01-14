//! Runtime CPU feature detection tool
//!
//! Prints all detectable CPU features and their availability on the current system.
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
}

#[cfg(target_arch = "aarch64")]
fn detect_aarch64_features() {
    println!("## aarch64 Features");
    println!();

    // Use std_detect crate macros
    macro_rules! check_feature {
        ($name:tt) => {
            let detected = std::arch::is_aarch64_feature_detected!($name);
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
    let has_dotprod = std::arch::is_aarch64_feature_detected!("dotprod")
        && std::arch::is_aarch64_feature_detected!("fp16");
    let has_apple_m1 = has_dotprod
        && std::arch::is_aarch64_feature_detected!("sha3")
        && std::arch::is_aarch64_feature_detected!("fcma");
    let has_sve2 = std::arch::is_aarch64_feature_detected!("sve2")
        && std::arch::is_aarch64_feature_detected!("i8mm")
        && std::arch::is_aarch64_feature_detected!("bf16");

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
}
