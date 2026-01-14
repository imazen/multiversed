# multiversed

Attribute macros wrapping [`multiversion`](https://crates.io/crates/multiversion) with predefined SIMD target presets.

## Usage

```rust
use multiversed::multiversed;

// Use cargo feature defaults (x86-64-v3, aarch64-baseline)
#[multiversed]
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

// Explicit presets
#[multiversed("x86-64-v4", "aarch64-sve2")]
pub fn optimized_sum(data: &[f32]) -> f32 {
    data.iter().sum()
}

// Mix presets with raw target strings
#[multiversed("x86-64-v3", "x86_64+avx2+fma+bmi2")]
pub fn custom_targets(data: &[f32]) -> f32 {
    data.iter().sum()
}
```

## Cargo Features (Presets)

Each feature is a complete, non-cumulative preset. Enable the ones you want.

### x86/x86_64

| Feature | Targets | Hardware |
|---------|---------|----------|
| `x86-64-v2` | SSE4.2 + POPCNT | Nehalem 2008+, most CPUs since ~2010 |
| `x86-64-v3` | AVX2 + FMA + BMI | Haswell 2013+, Zen 2 2019+ |
| `x86-64-v4` | AVX-512 | Skylake-X 2017+, Zen 4 2022+ |

### aarch64

| Feature | Targets | Hardware |
|---------|---------|----------|
| `aarch64-baseline` | NEON + crypto | All ARM64 |
| `aarch64-dotprod` | +dotprod +fp16 | Cortex-A75 2017+, Apple A11+ |
| `aarch64-crypto-ext` | +sha3 +fcma | Cortex-A76 2018+, Apple M1+ |
| `aarch64-sve2` | +SVE2 +i8mm +bf16 | Neoverse V1 2020+, Apple M4 2024+ |

### Examples

```toml
# Default: x86-64-v3 + aarch64-baseline
multiversed = "0.1"

# High-tier only
multiversed = { version = "0.1", default-features = false, features = ["x86-64-v4", "aarch64-sve2"] }

# Multiple tiers (runtime dispatch picks best)
multiversed = { version = "0.1", features = ["x86-64-v4"] }  # adds v4 to default v3
```

## Attribute Arguments

The `#[multiversed]` attribute accepts:

- **No arguments**: Uses targets from enabled cargo features
- **Preset names**: `"x86-64-v3"`, `"aarch64-baseline"`, etc.
- **Raw target strings**: Any string with `+` is passed directly to multiversion

Multiple arguments are comma-separated. All matching the same architecture are grouped together.

```rust
// Multiple presets for same arch = multiple dispatch targets
#[multiversed("x86-64-v3", "x86-64-v4")]
fn multi_tier(data: &[f32]) -> f32 { data.iter().sum() }

// Mixed presets and raw strings
#[multiversed("x86-64-v3", "x86_64+avx2+fma+custom")]
fn mixed(data: &[f32]) -> f32 { data.iter().sum() }
```

## How It Works

This crate generates `#[multiversion::multiversion(targets(...))]` attributes with
architecture-appropriate target strings. The actual code generation and runtime
dispatch are handled by the excellent `multiversion` crate.

Cross-compilation works correctly: cargo features control which targets are available,
while `#[cfg_attr]` in the generated code selects based on the actual target architecture.

## License

MIT OR Apache-2.0
