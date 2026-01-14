# multiversed

Attribute macros wrapping [`multiversion`](https://crates.io/crates/multiversion) with predefined SIMD target presets.

## Why?

Writing multiversion target strings is tedious and error-prone:

```rust
// Without multiversed - verbose and hard to maintain
#[multiversion::multiversion(targets(
    "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt+cmpxchg16b+avx+avx2+bmi1+bmi2+f16c+fma+lzcnt+movbe+xsave+fxsr+avx512f+avx512bw+avx512dq+avx512vl+avx512cd",
    "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt+cmpxchg16b+avx+avx2+bmi1+bmi2+f16c+fma+lzcnt+movbe+xsave+fxsr"
))]
fn sum(data: &[f32]) -> f32 { data.iter().sum() }

// With multiversed - clean preset names
#[multiversed("x86-64-v4", "x86-64-v3")]
fn sum(data: &[f32]) -> f32 { data.iter().sum() }
```

## Usage

```rust
use multiversed::multiversed;

// Use cargo feature defaults (x86-64-v3, aarch64-dotprod)
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

## Presets

Each preset is a complete, non-cumulative feature set based on the [x86-64 psABI] microarchitecture levels and ARM architecture versions.

[x86-64 psABI]: https://gitlab.com/x86-psABIs/x86-64-ABI

### x86-64

| Preset | Key Features | Hardware |
|--------|--------------|----------|
| `x86-64-v2` | SSE4.2, POPCNT | Nehalem 2008+, Bulldozer 2011+ |
| `x86-64-v3` | AVX2, FMA, BMI1/2 | Haswell 2013+, Zen 1 2017+ |
| `x86-64-v4` | AVX-512 (F/BW/DQ/VL/CD) | Xeon 2017+, Zen 4 2022+ |

> **Note**: Intel consumer CPUs (12th-15th gen: Alder Lake, Raptor Lake, Arrow Lake) do **not** have AVX-512 due to E-core limitations. Only Xeon servers, i9-X/Xeon-W workstations, and AMD Zen 4+ have AVX-512.

### aarch64

| Preset | Key Features | Hardware |
|--------|--------------|----------|
| `aarch64-dotprod` | dotprod, fp16 | Neoverse N1, Cortex-A75+, Apple M1+, Snapdragon X |
| `aarch64-apple-m1` | + sha3, fcma | Apple M1+, Snapdragon X, Neoverse V1+ |
| `aarch64-sve2` | + SVE2, i8mm, bf16 | Neoverse N2/V2+ (Graviton4, Grace, Axion) |

> **Note**: SVE/SVE2 is **server-only** (Neoverse). Apple Silicon, Qualcomm Oryon, and Cortex-A/X mobile cores do not implement SVE.

### Planned Presets

| Preset | Key Features | Hardware | Status |
|--------|--------------|----------|--------|
| `aarch64-sve` | SVE, i8mm, bf16 | Neoverse V1 (Graviton3) | Planned |

Graviton3 (Neoverse V1) has SVE but **not** SVE2. The `aarch64-sve` preset will target this gap.

## Dispatch Overhead

Benchmarks show **no measurable overhead** from feature string complexity:

| Configuration | Time (64 floats) |
|---------------|------------------|
| No multiversion | 15.4 ns |
| 1 feature | 15.8 ns |
| 27 features | 15.6 ns |
| 6 targets | 15.6 ns |

The ~0.3ns difference is the indirect call cost. Feature checking happens at compile time, not runtime.

## Cargo Features

```toml
# Default: x86-64-v3 + aarch64-dotprod
multiversed = "0.1"

# Server-focused (AVX-512 + SVE2)
multiversed = { version = "0.1", default-features = false, features = ["x86-64-v4", "aarch64-sve2"] }

# Multiple tiers (runtime dispatch picks best)
multiversed = { version = "0.1", features = ["x86-64-v4"] }  # adds v4 to default v3

# Disable multiversioning (debugging/profiling)
multiversed = { version = "0.1", features = ["force-disable"] }
```

### Special Features

| Feature | Description |
|---------|-------------|
| `force-disable` | Pass through functions unchanged. Useful for debugging or faster builds. |

## wasm32

The `multiversion` crate does not support wasm32 (no runtime feature detection).
For wasm32 SIMD, compile with the target feature directly:

```bash
RUSTFLAGS="-C target-feature=+simd128" cargo build --target wasm32-unknown-unknown
```

## Architecture Reference

See [ARCH_TABLE.md](ARCH_TABLE.md) for detailed CPU feature matrices covering:
- x86-64: Intel (Nehalem → Arrow Lake), AMD (Bulldozer → Zen 5)
- aarch64: Neoverse (N1/V1/N2/V2), Apple (M1-M5), Cortex (A75-X5), Qualcomm Oryon

## How It Works

This crate generates `#[multiversion::multiversion(targets(...))]` attributes with
architecture-appropriate target strings. The actual code generation and runtime
dispatch are handled by the excellent [`multiversion`](https://crates.io/crates/multiversion) crate.

Cross-compilation works correctly: cargo features control which targets are available,
while `#[cfg_attr]` in the generated code selects based on the actual target architecture.

## License

MIT OR Apache-2.0
