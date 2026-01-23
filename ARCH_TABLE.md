# CPU Architecture Feature Table

Reference document for multiversed presets and CPU feature availability.

## Quick Reference: Available Presets

| Preset | Features | Use Case |
|--------|----------|----------|
| `x86-64-v3` | AVX2, FMA, BMI | Desktop/laptop baseline (2013+) |
| `x86-64-v4` | + AVX-512 core | Skylake-X, Zen 4+ servers |
| `x86-64-v4-modern` | + VNNI, VBMI2, BF16, GFNI | Ice Lake+, Zen 4+ (default) |
| `arm64` | NEON, FP16 | All modern ARM64 (2017+) |

For features beyond these presets, use raw target strings: `"aarch64+neon+dotprod+sha3"`

## x86-64 Microarchitecture Levels (psABI Standard)

Source: [x86-64 psABI](https://gitlab.com/x86-psABIs/x86-64-ABI), [openSUSE Wiki](https://en.opensuse.org/X86-64_microarchitecture_levels)

Defined in 2020 by AMD, Intel, Red Hat, and SUSE. Each level is a superset of the previous.

| Level | Features Added | Total Features |
|-------|----------------|----------------|
| **v1** | (baseline) | CMOV, CX8, FPU, FXSR, MMX, SCE, SSE, SSE2 |
| **v2** | +SSE3/4, POPCNT | + CX16, LAHF-SAHF, POPCNT, SSE3, SSSE3, SSE4.1, SSE4.2 |
| **v3** | +AVX2, FMA, BMI | + AVX, AVX2, BMI1, BMI2, F16C, FMA, LZCNT, MOVBE, XSAVE |
| **v4** | +AVX-512 core | + AVX512F, AVX512BW, AVX512CD, AVX512DQ, AVX512VL |

Note: Crypto (AES-NI, SHA) and RDRAND excluded from level requirements.

## x86-64 Intel CPUs

| Microarch | Different names / different SKUs | Level | AVX-512 | Notes | Year |
|-----------|-----------|-------|---------|-------|------|
| **Nehalem** | Core i 1st gen | v2 | - | First SSE4.2 | 2008 |
| **Sandy Bridge** | Core i 2nd gen | v2 | - | First AVX | 2011 |
| **Haswell** | Core i 4th gen | v3 | - | First AVX2+FMA | 2013 |
| **Skylake** | Core i 6th gen | v3 | - | Consumer | 2015 |
| **Skylake-X** | Core X, Xeon W | v4 | ✓ Full | First AVX-512 desktop | 2017 |
| **Cascade Lake** | Xeon Scalable 2nd | v4 | ✓ Full | +VNNI | 2019 |
| **Ice Lake** | Core i 10th gen (mobile) | v4 | ✓ Full | Consumer AVX-512 | 2019 |
| **Rocket Lake** | Core i 11th gen | v4 | ✓ Full | Last consumer AVX-512 | 2021 |
| **Alder Lake** | Core i 12th gen | v3 | ❌ Fused off | E-cores lack AVX-512 | 2021 |
| **Raptor Lake** | Core i 13th/14th gen | v3 | ❌ Fused off | Same as Alder Lake | 2022 |
| **Sapphire Rapids** | Xeon Scalable 4th | v4+ | ✓ Full | +AMX, +BF16, +FP16 | 2023 |
| **Emerald Rapids** | Xeon Scalable 5th | v4+ | ✓ Full | Same as Sapphire | 2023 |
| **Arrow Lake** | Core Ultra 200 | v3 | ❌ Disabled | E-cores still lack AVX-512 | 2024 |
| **Granite Rapids** | Xeon 6 | v4+ | ✓ Full | +AMX-FP16 | 2024 |
| **Panther Lake** | (upcoming) | v4? | ✓ AVX10 | AVX10.1-512, both cores | 2025 |

**Key insight**: Intel consumer CPUs (Alder Lake through Arrow Lake) do NOT have AVX-512 due to E-core limitations. Only Xeon server and i9-X/Xeon W workstation have it.

## x86-64 AMD CPUs

| Microarch | Products | Level | AVX-512 | Notes | Year |
|-----------|----------|-------|---------|-------|------|
| **Bulldozer** | FX series | v2 | - | First AMD SSE4.2 | 2011 |
| **Jaguar** | APUs, consoles | v2 | - | PS4/Xbox One | 2013 |
| **Zen 1** | Ryzen 1000 | v3 | - | First Ryzen | 2017 |
| **Zen+** | Ryzen 2000 | v3 | - | | 2018 |
| **Zen 2** | Ryzen 3000, EPYC Rome | v3 | - | | 2019 |
| **Zen 3** | Ryzen 5000, EPYC Milan | v3 | - | | 2020 |
| **Zen 4** | Ryzen 7000, EPYC Genoa | v4 | ✓ 256b* | First AMD AVX-512 | 2022 |
| **Zen 5** | Ryzen 9000, EPYC Turin | v4 | ✓ 256b* | +VAES, +VPCLMULQDQ | 2024 |

*AMD implements AVX-512 with 256-bit execution units (double-pumped). Certain instructions may be slower than Intel's native 512-bit (e.g., vpcompressw).

## x86-64 AVX-512 Subsets

Not all AVX-512 CPUs have the same extensions:

| Extension | Zen 4/5 | Skylake-X | Ice Lake | Sapphire+ | Purpose |
|-----------|---------|-----------|----------|-----------|---------|
| **F** (Foundation) | ✓ | ✓ | ✓ | ✓ | Base |
| **CD** (Conflict Detect) | ✓ | ✓ | ✓ | ✓ | Scatter/gather |
| **BW** (Byte/Word) | ✓ | ✓ | ✓ | ✓ | 8/16-bit ops |
| **DQ** (Dword/Qword) | ✓ | ✓ | ✓ | ✓ | 32/64-bit ops |
| **VL** (Vector Length) | ✓ | ✓ | ✓ | ✓ | 128/256-bit |
| **VNNI** | ✓ | - | ✓ | ✓ | Neural network int8 |
| **IFMA** | ✓ | - | ✓ | ✓ | Int52 multiply-add |
| **VBMI** | ✓ | - | ✓ | ✓ | Byte permute |
| **VBMI2** | ✓ | - | ✓ | ✓ | Compress/expand |
| **BITALG** | ✓ | - | ✓ | ✓ | Bit manipulation |
| **VPOPCNTDQ** | ✓ | - | ✓ | ✓ | Population count |
| **BF16** | ✓ | - | - | ✓ | Bfloat16 (ML) |
| **FP16** | - | - | - | ✓ | IEEE float16 |
| **VP2INTERSECT** | - | - | - | - | Rare (Tiger Lake only) |

## x86-64 Feature Availability by Year

| Feature | Intel Consumer | Intel Server | AMD | Year |
|---------|----------------|--------------|-----|------|
| SSE4.2 | Nehalem+ | Nehalem+ | Bulldozer+ | 2008 |
| AVX | Sandy Bridge+ | Sandy Bridge+ | Bulldozer+ | 2011 |
| AVX2+FMA | Haswell+ | Haswell+ | Zen+ | 2013 |
| AVX-512 base | Rocket Lake only | Skylake-X+ | Zen 4+ | 2017 |
| AVX-512 VNNI | - | Cascade Lake+ | Zen 4+ | 2019 |
| AVX-512 BF16 | - | Cooper Lake+ | Zen 4+ | 2020 |
| AMX | - | Sapphire Rapids+ | - | 2023 |

## x86-64 Distribution Baselines

| Distribution | Baseline | Notes |
|--------------|----------|-------|
| Most Linux distros | v1 | Maximum compatibility |
| RHEL 9 | v2 | 2022+ |
| RHEL 10 | v3 | 2025+ |
| Gentoo (optional) | v3 | User choice |
| Clear Linux | v3+ | Performance-focused |
| Windows 11 | v1 | But requires TPM 2.0 |

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

## Implemented Presets

Based on the above data:

### x86-64
| Preset | Features | Target Hardware |
|--------|----------|-----------------|
| `x86-64-v2` | SSE4.2, POPCNT | 2008+ (broad compat) |
| `x86-64-v3` | AVX2, FMA, BMI | 2013+ (recommended) |
| `x86-64-v4` | AVX-512 (F/BW/DQ/VL/CD) | Skylake-X 2017+, Zen 4+ |
| `x86-64-v4-modern` | + VNNI, VBMI2, BF16, GFNI, VAES | Ice Lake 2019+, Zen 4+ |

### aarch64

| Preset | Key Features | Target Hardware |
|--------|--------------|-----------------|
| `arm64` | NEON, FP16 | A75+, M1+, N1+, Oryon (2017+) |

For additional aarch64 features (dotprod, sha3, SVE), use raw target strings.

## Sources

### x86-64
- [x86-64 psABI specification](https://gitlab.com/x86-psABIs/x86-64-ABI)
- [openSUSE x86-64 microarchitecture levels](https://en.opensuse.org/X86-64_microarchitecture_levels)
- [LLVM Zen 4 enablement](https://github.com/llvm/llvm-project/commit/1f057e365f1fdd630c023a990e84e95a6c792e4d)
- [LLVM Zen 5 enablement](https://github.com/llvm/llvm-project/commit/149a150b50c112e26fc5acbdd58250c44ccd777f)
- [AVX-512 Wikipedia](https://en.wikipedia.org/wiki/AVX-512)
- [Intel Alder Lake AVX-512 status](https://www.intel.com/content/www/us/en/support/articles/000089918/processors.html)

### aarch64
- [LLVM Neoverse V1 support](https://reviews.llvm.org/D90765)
- [LLVM Neoverse N2 commit](https://github.com/llvm/llvm-project/commit/2b6691894ab671706051a6d7ef54571546c20d3b)
- [LLVM Neoverse V2 support](https://reviews.llvm.org/D134352)
- [LLVM Apple CPU support](https://github.com/llvm/llvm-project/commit/677da09d0259d7530d32e85cb561bee15f0066e2)
- [LLVM Apple M5/A19](https://github.com/llvm/llvm-project/commit/f85494f6afeb)
- [ARM Neoverse V1 page](https://www.arm.com/products/silicon-ip-cpu/neoverse/neoverse-v1)
- [WikiChip Neoverse](https://en.wikichip.org/wiki/arm_holdings/neoverse)
