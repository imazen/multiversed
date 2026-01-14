//! Attribute macros wrapping `multiversion` with predefined SIMD target presets.
//!
//! This crate provides ergonomic attribute macros that wrap `#[multiversion::multiversion]`
//! with carefully curated target sets for each architecture.
//!
//! Target string constants are conditionally used based on features, so some may appear unused.
#![allow(dead_code)]

//! # Usage
//!
//! ```ignore
//! use multiversed::multiversion;
//!
//! #[multiversion]
//! pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
//!     a.iter().zip(b).map(|(x, y)| x * y).sum()
//! }
//! ```
//!
//! # Features
//!
//! Target tiers are additive. Higher tiers include all lower tier targets.
//!
//! ## x86/x86_64
//!
//! | Feature | Targets | Hardware |
//! |---------|---------|----------|
//! | `x86-v2` | SSE4.2 + POPCNT | Nehalem 2008+, most CPUs since ~2010 |
//! | `x86-v3` | AVX2 + FMA + BMI | Haswell 2013+, Zen 2 2019+ |
//! | `x86-v4` | AVX-512 | Skylake-X 2017+, Zen 4 2022+ |
//!
//! ## aarch64
//!
//! | Feature | Targets | Hardware |
//! |---------|---------|----------|
//! | `aarch64-baseline` | NEON + crypto | All ARM64 |
//! | `aarch64-dotprod` | +dotprod +fp16 | Cortex-A75 2017+, Apple A11+ |
//! | `aarch64-crypto-ext` | +sha3 +fcma | Cortex-A76 2018+, Apple M1+ |
//! | `aarch64-sve2` | +SVE2 +i8mm +bf16 | Neoverse V1 2020+, Apple M4 2024+ |
//!
//! ## Presets
//!
//! - `conservative` (default): x86-v3 + aarch64-baseline
//! - `extended`: x86-v4 + aarch64-dotprod
//! - `full`: x86-v4 + aarch64-sve2
//!
//! # Cross-compilation
//!
//! Features control which targets are *available*. The actual target selection
//! happens at compile time via `#[cfg_attr]`, so cross-compilation works correctly.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

// ============================================================================
// Target string definitions
// ============================================================================

// x86-64-v2: SSE4.2 baseline (Nehalem 2008+)
const X86_V2: &str = "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt";

// x86-64-v3: AVX2 + FMA (Haswell 2013+, Zen 2 2019+)
const X86_V3: &str = "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt+cmpxchg16b+avx+avx2+bmi1+bmi2+f16c+fma+lzcnt+movbe+xsave+fxsr";

// x86-64-v4: AVX-512 (Skylake-X 2017+, Zen 4 2022+)
const X86_V4: &str = "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt+cmpxchg16b+avx+avx2+bmi1+bmi2+f16c+fma+lzcnt+movbe+xsave+fxsr+avx512f+avx512bw+avx512dq+avx512vl+avx512cd+gfni+vaes+vpclmulqdq";

// aarch64 baseline: NEON + crypto (all ARM64)
const AARCH64_BASELINE: &str = "aarch64+neon+lse+aes+sha2+crc";

// aarch64 dotprod: +dotprod +fp16 (Cortex-A75 2017+, Apple A11+)
const AARCH64_DOTPROD: &str = "aarch64+neon+lse+aes+sha2+crc+dotprod+rcpc+fp16+fhm";

// aarch64 crypto-ext: +sha3 +fcma (Cortex-A76 2018+, Apple M1+)
const AARCH64_CRYPTO_EXT: &str = "aarch64+neon+lse+aes+sha2+sha3+crc+dotprod+rcpc+fp16+fhm+fcma";

// aarch64 SVE2: +SVE2 +i8mm +bf16 (Neoverse V1 2020+, Apple M4 2024+)
const AARCH64_SVE2: &str =
    "aarch64+neon+lse+aes+sha2+crc+dotprod+rcpc+fp16+fhm+sve2+sve2-bitperm+i8mm+bf16";

// ============================================================================
// Helper to build target list based on features
// ============================================================================

// These functions use conditional cfg! to build target lists.
// The vec init-then-push pattern is required for conditional compilation.
#[allow(clippy::vec_init_then_push)]
fn x86_targets() -> Vec<&'static str> {
    let mut targets = Vec::new();

    #[cfg(feature = "x86-v4")]
    targets.push(X86_V4);

    #[cfg(feature = "x86-v3")]
    targets.push(X86_V3);

    #[cfg(feature = "x86-v2")]
    targets.push(X86_V2);

    targets
}

#[allow(clippy::vec_init_then_push)]
fn aarch64_targets() -> Vec<&'static str> {
    let mut targets = Vec::new();

    #[cfg(feature = "aarch64-sve2")]
    targets.push(AARCH64_SVE2);

    #[cfg(feature = "aarch64-crypto-ext")]
    targets.push(AARCH64_CRYPTO_EXT);

    #[cfg(feature = "aarch64-dotprod")]
    targets.push(AARCH64_DOTPROD);

    #[cfg(feature = "aarch64-baseline")]
    targets.push(AARCH64_BASELINE);

    targets
}

/// Apply multiversion with targets based on enabled features.
///
/// This macro wraps `#[multiversion::multiversion]` with predefined target sets
/// based on crate features. The default features provide good coverage for
/// modern x86-64 and ARM64 hardware.
///
/// # Example
///
/// ```ignore
/// use multiversed::multiversion;
///
/// #[multiversion]
/// pub fn sum(data: &[f32]) -> f32 {
///     data.iter().sum()
/// }
/// ```
///
/// # Target Selection
///
/// Targets are selected based on enabled features:
///
/// - **x86-64**: Uses v4 (AVX-512), v3 (AVX2+FMA), or v2 (SSE4.2) based on features
/// - **aarch64**: Uses sve2, crypto-ext, dotprod, or baseline NEON based on features
/// - **Other architectures**: No multiversion (passthrough)
#[proc_macro_attribute]
pub fn multiversion(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);

    let x86_targets = x86_targets();
    let aarch64_targets = aarch64_targets();

    // Use cfg_attr to select the right attribute based on target architecture
    // This handles cross-compilation correctly
    let output = if x86_targets.is_empty() && aarch64_targets.is_empty() {
        // No features enabled - passthrough
        quote! { #func }
    } else if x86_targets.is_empty() {
        quote! {
            #[cfg_attr(target_arch = "aarch64", multiversion::multiversion(targets(#(#aarch64_targets),*)))]
            #func
        }
    } else if aarch64_targets.is_empty() {
        quote! {
            #[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), multiversion::multiversion(targets(#(#x86_targets),*)))]
            #func
        }
    } else {
        quote! {
            #[cfg_attr(
                any(target_arch = "x86", target_arch = "x86_64"),
                multiversion::multiversion(targets(#(#x86_targets),*))
            )]
            #[cfg_attr(
                target_arch = "aarch64",
                multiversion::multiversion(targets(#(#aarch64_targets),*))
            )]
            #func
        }
    };

    output.into()
}

/// Convenience alias for `#[multiversion]` with conservative targets.
///
/// Same as `#[multiversion]` - uses whatever features are enabled.
/// The name is provided for clarity when you want to emphasize
/// you're using the default/conservative preset.
#[proc_macro_attribute]
pub fn multiversion_conservative(attr: TokenStream, item: TokenStream) -> TokenStream {
    multiversion(attr, item)
}

/// Explicit x86-only multiversion.
///
/// Only generates x86/x86_64 variants, no ARM targets.
/// Useful when you have architecture-specific implementations.
#[proc_macro_attribute]
pub fn multiversion_x86(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let targets = x86_targets();

    if targets.is_empty() {
        return quote! { #func }.into();
    }

    quote! {
        #[cfg_attr(
            any(target_arch = "x86", target_arch = "x86_64"),
            multiversion::multiversion(targets(#(#targets),*))
        )]
        #func
    }
    .into()
}

/// Explicit aarch64-only multiversion.
///
/// Only generates ARM64 variants, no x86 targets.
/// Useful when you have architecture-specific implementations.
#[proc_macro_attribute]
pub fn multiversion_aarch64(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let targets = aarch64_targets();

    if targets.is_empty() {
        return quote! { #func }.into();
    }

    quote! {
        #[cfg_attr(
            target_arch = "aarch64",
            multiversion::multiversion(targets(#(#targets),*))
        )]
        #func
    }
    .into()
}
