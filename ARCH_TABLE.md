# CPU Architecture Feature Table

Working document for designing multiversed presets.

## x86-64 Microarchitecture Levels

| Level | Key Features | CPUs | Year |
|-------|--------------|------|------|
| **x86-64-v1** | SSE2 (baseline) | All x86-64 | 2003+ |
| **x86-64-v2** | SSE4.2, POPCNT | Nehalem, Bulldozer | 2008+ |
| **x86-64-v3** | AVX2, FMA, BMI1/2 | Haswell, Zen 2 | 2013+ |
| **x86-64-v4** | AVX-512F/BW/DQ/VL/CD | Skylake-X, Zen 4 | 2017+ |

## aarch64 Server (Neoverse)

Source: [LLVM target definitions](https://github.com/llvm/llvm-project)

| CPU | Arch | SVE | SVE2 | BF16 | I8MM | DOTPROD | Products | Year |
|-----|------|-----|------|------|------|---------|----------|------|
| **Neoverse N1** | v8.2 | - | - | - | - | ✓ | Graviton2, Ampere Altra | 2019 |
| **Neoverse V1** | v8.4 | 256b | - | ✓ | ✓ | ✓ | Graviton3 | 2021 |
| **Neoverse N2** | v9.0 | 128b | ✓ | ✓ | ✓ | ✓ | Cobalt 100, AmpereOne | 2022 |
| **Neoverse V2** | v9.0 | 128b | ✓ | ✓ | ✓ | ✓ | Graviton4, Grace, Axion | 2023 |
| **Neoverse N3** | v9.2 | ? | ✓ | ✓ | ✓ | ✓ | (upcoming) | 2025 |
| **Neoverse V3** | v9.2 | ? | ✓ | ✓ | ✓ | ✓ | (upcoming) | 2025 |

## aarch64 Apple Silicon

Source: [LLVM Apple CPU commits](https://github.com/llvm/llvm-project/commit/677da09d0259d7530d32e85cb561bee15f0066e2)

| Chip | A-series | Arch | BF16 | I8MM | SHA3 | SME | SME2 | Year |
|------|----------|------|------|------|------|-----|------|------|
| **M1** | A14 | v8.5 | - | - | ✓ | - | - | 2020 |
| **M2** | A15 | v8.6* | ✓ | ✓ | ✓ | - | - | 2022 |
| **M3** | A16 | v8.6* | ✓ | ✓ | ✓ | - | - | 2023 |
| **M4** | A18 | v8.7 | ✓ | ✓ | ✓ | ✓ | ✓ | 2024 |
| **M5** | A19 | v8.7+ | ✓ | ✓ | ✓ | ✓ | ✓ | 2025 |

*Marked v8.5 in LLVM to avoid SM4 crypto, but has v8.6 features (BF16, I8MM).

Note: Apple does NOT expose SVE/SVE2 via standard detection (uses proprietary AMX instead).

## aarch64 Mobile (Cortex / Snapdragon Phone)

| CPU | Arch | BF16 | I8MM | DOTPROD | SHA3 | Example SoC | Year |
|-----|------|------|------|---------|------|-------------|------|
| **Cortex-A75** | v8.2 | - | - | ✓ | - | Snapdragon 845 | 2017 |
| **Cortex-A76** | v8.2 | - | - | ✓ | - | Snapdragon 855 | 2018 |
| **Cortex-A77** | v8.2 | - | - | ✓ | - | Snapdragon 865 | 2019 |
| **Cortex-A78** | v8.2 | - | - | ✓ | - | Snapdragon 888 | 2020 |
| **Cortex-X1** | v8.2 | - | - | ✓ | - | Snapdragon 888 | 2020 |
| **Cortex-A710** | v9.0 | ✓ | ✓ | ✓ | ✓ | Snapdragon 8 Gen 1 | 2021 |
| **Cortex-X2** | v9.0 | ✓ | ✓ | ✓ | ✓ | Snapdragon 8 Gen 1 | 2021 |
| **Cortex-X3** | v9.0 | ✓ | ✓ | ✓ | ✓ | Snapdragon 8 Gen 2 | 2022 |
| **Cortex-X4** | v9.2 | ✓ | ✓ | ✓ | ✓ | Snapdragon 8 Gen 3 | 2023 |
| **Cortex-X5** | v9.2 | ✓ | ✓ | ✓ | ✓ | Snapdragon 8 Elite | 2024 |

Note: Cortex-A/X cores do NOT implement SVE/SVE2 (NEON only).

## aarch64 Laptop (Qualcomm Oryon / Snapdragon X)

Source: Qualcomm Oryon is custom, not Cortex-based

| Chip | Arch | SVE | BF16 | I8MM | SHA3 | FCMA | Products | Year |
|------|------|-----|------|------|------|------|----------|------|
| **Oryon Gen 1** | v8.7 | - | ✓ | ✓ | ✓ | ✓ | Snapdragon X Elite/Plus | 2024 |
| **Oryon Gen 2** | v9? | - | ✓ | ✓ | ✓ | ✓ | Snapdragon X2 (expected) | 2025 |

Note: Qualcomm chose NOT to implement SVE in Oryon cores.

## Feature Availability Summary

| Feature | Arch | Server (Neoverse) | Mobile (Cortex) | Apple | Oryon |
|---------|------|-------------------|-----------------|-------|-------|
| NEON | v8.0 | All | All | All | All |
| DOTPROD | v8.2 | N1+ | A75+ | A11+ | ✓ |
| FP16 | v8.2 | N1+ | A75+ | A11+ | ✓ |
| SHA3 | v8.2 | V1+ | A710+ | A12+ | ✓ |
| FCMA | v8.3 | V1+ | A710+ | A12+ | ✓ |
| BF16 | v8.6 | V1, N2+ | A710+ | M2+ | ✓ |
| I8MM | v8.6 | V1, N2+ | A710+ | M2+ | ✓ |
| SVE | v8.2 | V1, N2+ | - | - | - |
| SVE2 | v9.0 | N2, V2+ | - | - | - |
| SME | v9.0 | V3+ | - | M4+ | - |
| SME2 | v9.2 | V3+ | - | M4+ | - |

## Proposed Presets

Based on the above data:

### x86-64
| Preset | Features | Target Hardware |
|--------|----------|-----------------|
| `x86-64-v2` | SSE4.2, POPCNT | 2008+ (broad compat) |
| `x86-64-v3` | AVX2, FMA, BMI | 2013+ (recommended) |
| `x86-64-v4` | AVX-512 | 2017+ (server/Zen4+) |

### aarch64

| Preset | Key Features | Target Hardware |
|--------|--------------|-----------------|
| `aarch64-baseline` | NEON, DOTPROD, FP16 | All 2017+ (N1, A75+, M1+, Oryon) |
| `aarch64-v84` | + SHA3, FCMA | M1+, Oryon, V1+ (2020+) |
| `aarch64-v86` | + BF16, I8MM | M2+, Oryon, V1, N2+, A710+ (2021+) |
| `aarch64-sve` | + SVE (no SVE2) | Graviton3 (V1) only |
| `aarch64-sve2` | + SVE2 | N2, V2+, (M4 not exposed) |

### Alternative naming (more intuitive?)

| Preset | Key Features | Target Hardware |
|--------|--------------|-----------------|
| `aarch64-2017` | DOTPROD, FP16 | N1, A75+, M1 |
| `aarch64-2020` | + SHA3, FCMA | M1+, Oryon, V1+ |
| `aarch64-2022` | + BF16, I8MM | M2+, Oryon, V1, N2+, A710+ |
| `aarch64-sve` | + SVE | Graviton3 |
| `aarch64-sve2` | + SVE2 | Graviton4+, (M4) |

## Open Questions

1. Should we have an `aarch64-apple` preset separate from server? (Apple has SHA3/FCMA but not SVE)
2. Is `aarch64-v86` (BF16+I8MM) worth a preset? It covers M2+, Oryon, newer Cortex
3. Should baseline include DOTPROD? (Some very old ARMv8.0 chips don't have it)
4. How to handle M4's SME vs server SVE2 - different vector approaches

## Sources

- [LLVM Neoverse V1 support](https://reviews.llvm.org/D90765)
- [LLVM Neoverse N2 commit](https://github.com/llvm/llvm-project/commit/2b6691894ab671706051a6d7ef54571546c20d3b)
- [LLVM Neoverse V2 support](https://reviews.llvm.org/D134352)
- [LLVM Apple CPU support](https://github.com/llvm/llvm-project/commit/677da09d0259d7530d32e85cb561bee15f0066e2)
- [LLVM Apple M5/A19](https://github.com/llvm/llvm-project/commit/f85494f6afeb)
- [ARM Neoverse V1 page](https://www.arm.com/products/silicon-ip-cpu/neoverse/neoverse-v1)
- [WikiChip Neoverse](https://en.wikichip.org/wiki/arm_holdings/neoverse)
