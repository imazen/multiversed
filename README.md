# multiversed

Attribute macros wrapping [`multiversion`](https://crates.io/crates/multiversion) with predefined SIMD target presets.

## Why?

Writing multiversion target strings is tedious and error-prone:

```rust
// Without multiversed - verbose and hard to maintain
#[multiversion::multiversion(targets(
    "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt+cmpxchg16b+avx+avx2+fma+bmi1+bmi2+f16c+lzcnt+movbe+avx512f+avx512bw+avx512cd+avx512dq+avx512vl",
    "x86_64+sse+sse2+sse3+ssse3+sse4.1+sse4.2+popcnt+cmpxchg16b+avx+avx2+fma+bmi1+bmi2+f16c+lzcnt+movbe"
))]
fn sum(data: &[f32]) -> f32 { data.iter().sum() }

// With multiversed - clean preset names
#[multiversed("x86-64-v4", "x86-64-v3")]
fn sum(data: &[f32]) -> f32 { data.iter().sum() }
```

## Usage

```rust
use multiversed::multiversed;

// Use cargo feature defaults (x86-64-v3, x86-64-v4x, arm64-v2, wasm32-simd128)
#[multiversed]
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

// Explicit presets
#[multiversed("x86-64-v4", "arm64-v2")]
pub fn optimized_sum(data: &[f32]) -> f32 {
    data.iter().sum()
}

// Multiple tiers - runtime picks best available
#[multiversed("x86-64-v4", "x86-64-v3", "x86-64-v2")]
pub fn tiered_dispatch(data: &[f32]) -> f32 {
    data.iter().sum()
}

// Raw target strings (any string containing '+')
#[multiversed("x86_64+avx2+fma")]
pub fn custom_x86(data: &[f32]) -> f32 {
    data.iter().sum()
}

// Mix presets with raw target strings
#[multiversed("x86-64-v3", "x86_64+avx512f+avx512vbmi2", "arm64")]
pub fn mixed_targets(data: &[f32]) -> f32 {
    data.iter().sum()
}
```

## Presets

Feature lists match the [archmage token registry] — the source of truth. Each preset is a complete, non-cumulative feature set based on the [x86-64 psABI] microarchitecture levels and ARM architecture versions.

[x86-64 psABI]: https://gitlab.com/x86-psABIs/x86-64-ABI
[archmage token registry]: https://github.com/imazen/archmage

### x86-64

| Preset | Archmage Token | Key Features | Hardware |
|--------|----------------|--------------|----------|
| `x86-64-v2` | X64V2Token | SSE4.2, POPCNT | Nehalem 2008+, Bulldozer 2011+ |
| `x86-64-v3` | X64V3Token | AVX2, FMA, BMI1/2 | Haswell 2013+, Zen 1 2017+ |
| `x86-64-v4` | X64V4Token | AVX-512 (F/BW/DQ/VL/CD) | Skylake-X 2017+, Zen 4 2022+ |
| `x86-64-v4-modern` / `x86-64-v4x` | X64V4xToken | + VNNI, VBMI2, GFNI, VAES | Ice Lake 2019+, Zen 4 2022+ |

> **Note**: Intel consumer CPUs (12th-15th gen: Alder Lake, Raptor Lake, Arrow Lake) do **not** have AVX-512 due to E-core limitations. Only Xeon servers, i9-X/Xeon-W workstations, and AMD Zen 4+ have AVX-512. Use `x86-64-v4` for Skylake-X compatibility, or `x86-64-v4x` for Ice Lake+ / Zen 4+ only.

### aarch64

| Preset | Archmage Token | Key Features | Hardware |
|--------|----------------|--------------|----------|
| `arm64` / `arm64-v2` | Arm64V2Token | NEON, CRC, DotProd, FP16, AES | Cortex-A55+, Apple M1+, Graviton 2+ |
| `arm64-v3` | Arm64V3Token | + SHA3, I8MM, BF16 | Cortex-A510+, Apple M2+, Graviton 3+ |

### wasm32

| Preset | Archmage Token | Notes |
|--------|----------------|-------|
| `wasm32-simd128` | Wasm128Token | No-op (multiversion elides on wasm32) |

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
# Default: x86-64-v3 + x86-64-v4x + arm64-v2 + wasm32-simd128
multiversed = "0.3"

# Minimal (v3 only, no AVX-512)
multiversed = { version = "0.3", default-features = false, features = ["x86-64-v3", "arm64-v2"] }

# Add Skylake-X compatibility (base v4 without Ice Lake extras)
multiversed = { version = "0.3", features = ["x86-64-v4"] }

# Disable multiversioning (debugging/profiling)
multiversed = { version = "0.3", features = ["force-disable"] }
```

### Special Features

| Feature | Description |
|---------|-------------|
| `force-disable` | Pass through functions unchanged. Useful for debugging or faster builds. |

## wasm32

The `multiversion` crate elides itself on wasm32 (no runtime feature detection available), so there's no downside to using `#[multiversed]` in cross-platform code—it simply becomes a no-op on wasm.

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
