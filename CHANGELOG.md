# Changelog

## 0.3.0

Breaking changes to defaults; no API changes.

### Changed

- **Feature strings aligned with [archmage token-registry.toml](https://github.com/imazen/archmage).** All preset feature lists now match the archmage token definitions exactly. Removed `xsave` and `fxsr` from x86 presets (OS-level state management, always present when AVX is detected, no codegen impact). Removed `avx512bf16` from v4x (workload-specific, not part of X64V4xToken).
- **`arm64` is now an alias for `arm64-v2`** (Arm64V2Token: neon+crc+rdm+dotprod+fp16+aes+sha2). In 0.2.x, `arm64` resolved to bare `neon+fp16`. The new behavior is strictly wider — existing code gets more features, not fewer. Users who relied on the old minimal NEON set can use a raw target string: `"aarch64+neon"`.
- **Default features changed** from `["x86-64-v3", "x86-64-v4-modern", "arm64"]` to `["x86-64-v3", "x86-64-v4x", "arm64-v2", "wasm32-simd128"]`. The `arm64-v2` default gives the same effective targets as before (since `arm64` now aliases to `arm64-v2`). `wasm32-simd128` is a no-op (multiversion elides on wasm32) but signals cross-platform intent.

### Added

- **`x86-64-v4x`** alias for `x86-64-v4-modern`. Both names resolve to X64V4xToken. `x86-64-v4x` is the canonical cargo feature name; `x86-64-v4-modern` activates it.
- **`arm64-v2`** preset (Arm64V2Token): NEON + CRC + RDM + DotProd + FP16 + AES + SHA2. Targets Cortex-A55+, Apple M1+, Graviton 2+.
- **`arm64-v3`** preset (Arm64V3Token): v2 + FHM + FCMA + SHA3 + I8MM + BF16. Targets Cortex-A510+, Apple M2+, Graviton 3+.
- **`wasm32-simd128`** cargo feature and preset name. Multiversion elides on wasm32, so this is a no-op — but it compiles cleanly and lets cross-platform code use `#[multiversed("wasm32-simd128")]` without conditional compilation.
- **Runtime feature detection test** (`tests/runtime_report.rs`): diagnostic test that reports which presets match the current CPU.
- **wasm32-wasip1 test crate** (`test-crates/wasm-simd128/`): runs under wasmtime to verify compilation and correctness on wasm32.
- **CI**: wasm32 build + test steps (with and without simd128), additional aarch64 build combinations, ARM runners (ubuntu-24.04-arm, windows-11-arm).

### Fixed

- **Duplicate target deduplication.** When aliases resolve to the same target string (e.g., `"arm64"` and `"arm64-v2"` both resolve to Arm64V2Token), the macro now deduplicates them. Previously this caused a multiversion name collision on aarch64 targets.

## 0.2.0

Initial release with x86-64-v2/v3/v4/v4-modern and arm64 presets.
