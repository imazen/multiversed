#![deny(unsafe_code)]

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
//! // Use targets from enabled cargo features (default: x86-64-v3, x86-64-v4-modern, arm64-v2)
//! #[multiversed]
//! pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
//!     a.iter().zip(b).map(|(x, y)| x * y).sum()
//! }
//!
//! // Explicit presets
//! #[multiversed("x86-64-v4", "arm64-v2")]
//! pub fn optimized_sum(data: &[f32]) -> f32 {
//!     data.iter().sum()
//! }
//!
//! // Mix presets with custom raw target strings
//! #[multiversed("x86-64-v3", "aarch64+neon+dotprod")]
//! pub fn custom_targets(data: &[f32]) -> f32 {
//!     data.iter().sum()
//! }
//! ```
//!
//! # Cargo Features (Presets)
//!
//! Feature lists match the [archmage token registry] — the source of truth.
//! Each feature is a complete, non-cumulative preset based on the [x86-64 psABI]
//! microarchitecture levels and ARM architecture versions.
//!
//! [x86-64 psABI]: https://gitlab.com/x86-psABIs/x86-64-ABI
//! [archmage token registry]: https://github.com/imazen/archmage
//!
//! ## x86/x86_64
//!
//! | Feature | Archmage Token | Key Features | Hardware |
//! |---------|----------------|--------------|----------|
//! | `x86-64-v2` | X64V2Token | SSE4.2, POPCNT | Nehalem 2008+, Bulldozer 2011+ |
//! | `x86-64-v3` | X64V3Token | AVX2, FMA, BMI1/2 | Haswell 2013+, Zen 1 2017+ |
//! | `x86-64-v4` | X64V4Token | AVX-512 (F/BW/DQ/VL/CD) | Skylake-X 2017+, Zen 4 2022+ |
//! | `x86-64-v4-modern` | X64V4xToken | + VNNI, VBMI2, GFNI, VAES | Ice Lake 2019+, Zen 4 2022+ |
//!
//! **Note**: Intel consumer CPUs (Alder Lake 12th gen through Arrow Lake) do NOT have
//! AVX-512 due to E-core limitations. Only Xeon server, i9-X workstation, and AMD Zen 4+
//! have AVX-512.
//!
//! ## aarch64
//!
//! | Feature | Archmage Token | Key Features | Hardware |
//! |---------|----------------|--------------|----------|
//! | `arm64` | NeonToken | NEON | All AArch64 |
//! | `arm64-v2` | Arm64V2Token | + CRC, DotProd, FP16, AES | Cortex-A55+, Apple M1+, Graviton 2+ |
//! | `arm64-v3` | Arm64V3Token | + SHA3, I8MM, BF16 | Cortex-A510+, Apple M2+, Graviton 3+ |
//!
//! # Attribute Arguments
//!
//! The `#[multiversed]` attribute accepts:
//! - **No arguments**: Uses targets from enabled cargo features
//! - **Preset names**: `"x86-64-v3"`, `"arm64"`, etc.
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
//
// Feature lists match archmage token-registry.toml — the source of truth.
// ============================================================================

// x86-64-v2: SSE4.2 + POPCNT + CX16 (Nehalem 2008+, Bulldozer 2011+)
// Matches archmage X64V2Token
const X86_64_V2: &str = "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt+cmpxchg16b";

// x86-64-v3: AVX2 + FMA (Haswell 2013+, Zen 1 2017+)
// Matches archmage X64V3Token
const X86_64_V3: &str =
    "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt+cmpxchg16b+avx+avx2+fma+bmi1+bmi2+f16c+lzcnt+movbe";

// x86-64-v4: AVX-512 (Skylake-X 2017+, Zen 4 2022+)
// Matches archmage X64V4Token — pure psABI v4: F+CD+VL+DQ+BW only
const X86_64_V4: &str =
    "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt+cmpxchg16b+avx+avx2+fma+bmi1+bmi2+f16c+lzcnt+movbe+avx512f+avx512bw+avx512cd+avx512dq+avx512vl";

// x86-64-v4-modern: Full modern AVX-512 (Ice Lake 2019+, Zen 4 2022+)
// Matches archmage X64V4xToken — adds VNNI, VBMI2, BITALG, GFNI, VAES, VPCLMULQDQ
const X86_64_V4_MODERN: &str =
    "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt+cmpxchg16b+avx+avx2+fma+bmi1+bmi2+f16c+lzcnt+movbe+avx512f+avx512bw+avx512cd+avx512dq+avx512vl+avx512vpopcntdq+avx512ifma+avx512vbmi+avx512vbmi2+avx512bitalg+avx512vnni+vpclmulqdq+gfni+vaes";

// arm64: NEON baseline (all AArch64)
// Matches archmage NeonToken
const ARM64: &str = "aarch64+neon";

// arm64-v2: Modern ARM baseline (Cortex-A55+, Apple M1+, Graviton 2+)
// Matches archmage Arm64V2Token
const ARM64_V2: &str = "aarch64+neon+crc+rdm+dotprod+fp16+aes+sha2";

// arm64-v3: Full modern ARM SIMD (Cortex-A510+, Apple M2+, Graviton 3+)
// Matches archmage Arm64V3Token
const ARM64_V3: &str = "aarch64+neon+crc+rdm+dotprod+fp16+aes+sha2+fhm+fcma+sha3+i8mm+bf16";

// Note: wasm32 has no runtime feature detection and multiversion doesn't support it.
// The wasm32-simd128 feature exists for documentation but generates no multiversion code.
// Users must compile with -C target-feature=+simd128 for SIMD on wasm32.

// ============================================================================
// Preset name resolution
// ============================================================================

/// Resolve a preset name to its target string, or return the input as-is if it's a raw target.
fn resolve_target(s: &str) -> Option<&str> {
    match s {
        // x86 presets (psABI standard)
        "x86-64-v2" => Some(X86_64_V2),
        "x86-64-v3" => Some(X86_64_V3),
        "x86-64-v4" => Some(X86_64_V4),
        "x86-64-v4-modern" => Some(X86_64_V4_MODERN),
        // aarch64 presets
        "arm64" => Some(ARM64),
        "arm64-v2" => Some(ARM64_V2),
        "arm64-v3" => Some(ARM64_V3),
        // wasm32 - multiversion doesn't support it, ignore
        "wasm32-simd128" => None,
        // Raw target string - pass through if it looks like a valid target
        s if s.contains('+') && !s.starts_with("wasm32") => Some(s),
        // Unknown or wasm32 raw target - ignore
        _ => None,
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
    #[cfg(feature = "x86-64-v4-modern")]
    targets.push(X86_64_V4_MODERN);

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
    #[cfg(feature = "arm64-v3")]
    targets.push(ARM64_V3);

    #[cfg(feature = "arm64-v2")]
    targets.push(ARM64_V2);

    #[cfg(feature = "arm64")]
    targets.push(ARM64);

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
/// #[multiversed("x86-64-v4", "arm64-v2")]
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
/// - **Preset names**: `"x86-64-v3"`, `"arm64"`, etc.
/// - **Raw target strings**: Any string with `+` is passed through to multiversion
#[proc_macro_attribute]
pub fn multiversed(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);

    // force-disable feature: passthrough without any multiversion
    #[cfg(feature = "force-disable")]
    {
        let _ = attr; // suppress unused warning
        #[allow(clippy::needless_return)]
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
    let (x86_targets, aarch64_targets) = if args.targets.is_empty() {
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
        // Note: wasm32 targets are filtered out (multiversion doesn't support wasm32)
        let resolved: Vec<String> = args
            .targets
            .iter()
            .filter_map(|s| resolve_target(s).map(String::from))
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

    // Build cfg_attr for each architecture that has targets
    let x86_attr = if x86_targets.is_empty() {
        quote! {}
    } else {
        quote! {
            #[cfg_attr(
                any(target_arch = "x86", target_arch = "x86_64"),
                multiversion::multiversion(targets(#(#x86_targets),*))
            )]
        }
    };

    let aarch64_attr = if aarch64_targets.is_empty() {
        quote! {}
    } else {
        quote! {
            #[cfg_attr(
                target_arch = "aarch64",
                multiversion::multiversion(targets(#(#aarch64_targets),*))
            )]
        }
    };

    quote! {
        #x86_attr
        #aarch64_attr
        #func
    }
    .into()
}
