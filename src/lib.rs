//! Attribute macros wrapping `multiversion` with predefined SIMD target presets.
//!
//! This crate provides the `#[multiversed]` attribute that wraps `#[multiversion::multiversion]`
//! with carefully curated target sets for each architecture.
//!
//! # Usage
//!
//! ```ignore
//! use multiversed::multiversed;
//!
//! // Use targets from enabled cargo features (default: x86-64-v3, aarch64-baseline)
//! #[multiversed]
//! pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
//!     a.iter().zip(b).map(|(x, y)| x * y).sum()
//! }
//!
//! // Explicit presets
//! #[multiversed("x86-64-v4", "aarch64-sve2")]
//! pub fn optimized_sum(data: &[f32]) -> f32 {
//!     data.iter().sum()
//! }
//!
//! // Mix presets with custom raw target strings
//! #[multiversed("x86-64-v3", "x86_64+avx2+fma+bmi2")]
//! pub fn custom_targets(data: &[f32]) -> f32 {
//!     data.iter().sum()
//! }
//! ```
//!
//! # Cargo Features (Presets)
//!
//! Each feature is a complete, non-cumulative preset.
//!
//! ## x86/x86_64
//!
//! | Feature | Targets | Hardware |
//! |---------|---------|----------|
//! | `x86-64-v2` | SSE4.2 + POPCNT | Nehalem 2008+, most CPUs since ~2010 |
//! | `x86-64-v3` | AVX2 + FMA + BMI | Haswell 2013+, Zen 2 2019+ |
//! | `x86-64-v4` | AVX-512 | Skylake-X 2017+, Zen 4 2022+ |
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
//! # Attribute Arguments
//!
//! The `#[multiversed]` attribute accepts:
//! - **No arguments**: Uses targets from enabled cargo features
//! - **Preset names**: `"x86-64-v3"`, `"aarch64-baseline"`, etc.
//! - **Raw target strings**: Any string containing `+` is passed through as-is
//!
//! Multiple arguments are comma-separated and all are included in the target list.

#![allow(dead_code)]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, ItemFn, LitStr, Token};

// ============================================================================
// Target string definitions (preset name -> multiversion target string)
// ============================================================================

// x86-64-v2: SSE4.2 baseline (Nehalem 2008+)
const X86_64_V2: &str = "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt";

// x86-64-v3: AVX2 + FMA (Haswell 2013+, Zen 2 2019+)
const X86_64_V3: &str =
    "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt+cmpxchg16b+avx+avx2+bmi1+bmi2+f16c+fma+lzcnt+movbe+xsave+fxsr";

// x86-64-v4: AVX-512 (Skylake-X 2017+, Zen 4 2022+)
const X86_64_V4: &str =
    "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt+cmpxchg16b+avx+avx2+bmi1+bmi2+f16c+fma+lzcnt+movbe+xsave+fxsr+avx512f+avx512bw+avx512dq+avx512vl+avx512cd+gfni+vaes+vpclmulqdq";

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
// Preset name resolution
// ============================================================================

/// Resolve a preset name to its target string, or return the input as-is if it's a raw target.
fn resolve_target(s: &str) -> &str {
    match s {
        // x86 presets
        "x86-64-v2" => X86_64_V2,
        "x86-64-v3" => X86_64_V3,
        "x86-64-v4" => X86_64_V4,
        // aarch64 presets
        "aarch64-baseline" => AARCH64_BASELINE,
        "aarch64-dotprod" => AARCH64_DOTPROD,
        "aarch64-crypto-ext" => AARCH64_CRYPTO_EXT,
        "aarch64-sve2" => AARCH64_SVE2,
        // Raw target string (contains +, pass through as-is)
        _ => s,
    }
}

/// Check if a target string is for x86/x86_64 architecture.
fn is_x86_target(s: &str) -> bool {
    s.starts_with("x86_64+") || s.starts_with("x86+") || s.starts_with("x86-64-")
}

/// Check if a target string is for aarch64 architecture.
fn is_aarch64_target(s: &str) -> bool {
    s.starts_with("aarch64+") || s.starts_with("aarch64-")
}

// ============================================================================
// Attribute argument parsing
// ============================================================================

struct MultiversedArgs {
    targets: Vec<String>,
}

impl Parse for MultiversedArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut targets = Vec::new();

        while !input.is_empty() {
            let lit: LitStr = input.parse()?;
            targets.push(lit.value());

            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            }
        }

        Ok(MultiversedArgs { targets })
    }
}

// ============================================================================
// Default targets from cargo features
// ============================================================================

// Vec init-then-push pattern required for conditional cfg compilation.
#[allow(clippy::vec_init_then_push)]
fn default_x86_targets() -> Vec<&'static str> {
    let mut targets = Vec::new();

    // Higher tiers first (more specific optimizations)
    #[cfg(feature = "x86-64-v4")]
    targets.push(X86_64_V4);

    #[cfg(feature = "x86-64-v3")]
    targets.push(X86_64_V3);

    #[cfg(feature = "x86-64-v2")]
    targets.push(X86_64_V2);

    targets
}

#[allow(clippy::vec_init_then_push)]
fn default_aarch64_targets() -> Vec<&'static str> {
    let mut targets = Vec::new();

    // Higher tiers first (more specific optimizations)
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

// ============================================================================
// Main attribute macro
// ============================================================================

/// Apply multiversion with SIMD target presets.
///
/// # Usage
///
/// ```ignore
/// use multiversed::multiversed;
///
/// // Use cargo feature defaults
/// #[multiversed]
/// fn sum(data: &[f32]) -> f32 {
///     data.iter().sum()
/// }
///
/// // Explicit presets
/// #[multiversed("x86-64-v4", "aarch64-sve2")]
/// fn optimized(data: &[f32]) -> f32 {
///     data.iter().sum()
/// }
///
/// // Mix presets with raw target strings
/// #[multiversed("x86-64-v3", "x86_64+avx2+fma+custom")]
/// fn custom(data: &[f32]) -> f32 {
///     data.iter().sum()
/// }
/// ```
///
/// # Arguments
///
/// - **No arguments**: Uses targets from enabled cargo features
/// - **Preset names**: `"x86-64-v3"`, `"aarch64-baseline"`, etc.
/// - **Raw target strings**: Any string with `+` is passed through to multiversion
#[proc_macro_attribute]
pub fn multiversed(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);

    // force-disable feature: passthrough without any multiversion
    #[cfg(feature = "force-disable")]
    {
        let _ = attr; // suppress unused warning
        return quote! { #func }.into();
    }

    #[cfg(not(feature = "force-disable"))]
    {
        let args = parse_macro_input!(attr as MultiversedArgs);
        multiversed_impl(args, func)
    }
}

fn multiversed_impl(args: MultiversedArgs, func: ItemFn) -> TokenStream {
    // Collect targets, separating by architecture
    let (x86_targets, aarch64_targets): (Vec<String>, Vec<String>) = if args.targets.is_empty() {
        // No explicit targets - use cargo feature defaults
        let x86: Vec<String> = default_x86_targets()
            .into_iter()
            .map(String::from)
            .collect();
        let aarch64: Vec<String> = default_aarch64_targets()
            .into_iter()
            .map(String::from)
            .collect();
        (x86, aarch64)
    } else {
        // Explicit targets - resolve presets and partition by architecture
        let resolved: Vec<String> = args
            .targets
            .iter()
            .map(|s| resolve_target(s).to_string())
            .collect();

        let x86: Vec<String> = resolved
            .iter()
            .filter(|s| is_x86_target(s))
            .cloned()
            .collect();
        let aarch64: Vec<String> = resolved
            .iter()
            .filter(|s| is_aarch64_target(s))
            .cloned()
            .collect();

        (x86, aarch64)
    };

    // Generate output based on which architectures have targets
    let output = match (x86_targets.is_empty(), aarch64_targets.is_empty()) {
        (true, true) => {
            // No targets - passthrough without multiversion
            quote! { #func }
        }
        (false, true) => {
            // x86 only
            quote! {
                #[cfg_attr(
                    any(target_arch = "x86", target_arch = "x86_64"),
                    multiversion::multiversion(targets(#(#x86_targets),*))
                )]
                #func
            }
        }
        (true, false) => {
            // aarch64 only
            quote! {
                #[cfg_attr(
                    target_arch = "aarch64",
                    multiversion::multiversion(targets(#(#aarch64_targets),*))
                )]
                #func
            }
        }
        (false, false) => {
            // Both architectures
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
        }
    };

    output.into()
}
