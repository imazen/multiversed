# multiversed

Attribute macros wrapping [`multiversion`](https://crates.io/crates/multiversion) with predefined SIMD target presets.

## Usage

```rust
use multiversed::multiversion;

#[multiversion]
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}
```

This generates optimized versions for:
- **x86-64**: AVX2+FMA (Haswell 2013+, Zen 2 2019+)
- **aarch64**: NEON+crypto (all ARM64)

With runtime dispatch to the best available version.

## Features

Target tiers are additive. Higher tiers include all lower tier targets.

### x86/x86_64

| Feature | Targets | Hardware |
|---------|---------|----------|
| `x86-v2` | SSE4.2 + POPCNT | Nehalem 2008+, most CPUs since ~2010 |
| `x86-v3` | AVX2 + FMA + BMI | Haswell 2013+, Zen 2 2019+ |
| `x86-v4` | AVX-512 | Skylake-X 2017+, Zen 4 2022+ |

### aarch64

| Feature | Targets | Hardware |
|---------|---------|----------|
| `aarch64-baseline` | NEON + crypto | All ARM64 |
| `aarch64-dotprod` | +dotprod +fp16 | Cortex-A75 2017+, Apple A11+ |
| `aarch64-crypto-ext` | +sha3 +fcma | Cortex-A76 2018+, Apple M1+ |
| `aarch64-sve2` | +SVE2 +i8mm +bf16 | Neoverse V1 2020+, Apple M4 2024+ |

### Presets

```toml
# Conservative (default): x86-v3 + aarch64-baseline
multiversed = "0.1"

# Extended: x86-v4 + aarch64-dotprod
multiversed = { version = "0.1", features = ["extended"] }

# Full: x86-v4 + aarch64-sve2
multiversed = { version = "0.1", features = ["full"] }

# Custom: pick exactly what you want
multiversed = { version = "0.1", default-features = false, features = ["x86-v4", "aarch64-dotprod"] }
```

## Available Macros

- `#[multiversion]` - Uses enabled features for both architectures
- `#[multiversion_x86]` - Only generates x86/x86_64 variants
- `#[multiversion_aarch64]` - Only generates ARM64 variants

## How It Works

This crate generates `#[multiversion::multiversion(targets(...))]` attributes with
architecture-appropriate target strings. The actual code generation and runtime
dispatch are handled by the excellent `multiversion` crate.

Cross-compilation works correctly: features control which targets are *available*,
while `#[cfg_attr]` in the generated code selects based on the actual target architecture.

## License

MIT OR Apache-2.0
