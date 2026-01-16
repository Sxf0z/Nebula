# SpecterScript Performance

> v0.9.0 - Bytecode VM with NaN-boxed values

---

## Benchmark Results

Warm execution (5-run average, first run discarded).

| Benchmark | SpecterScript VM | Python 3.12 | Speedup |
|-----------|-----------------|-------------|---------|
| bench_loop (100K) | 10.4ms | 98.2ms | **9.5x** |
| bench_math (10K) | 6.8ms | 94.9ms | **14x** |

---

## Internals

### NaN-Boxing
All primitives packed into 64-bit values:
- Numbers: direct f64
- Integers: 48-bit signed
- Booleans: special NaN patterns
- Pointers: heap addresses

### Indexed Globals
O(1) global variable access via `Vec<Value>`.

### Specialized Opcodes
- `LoadLocal0-2`: no operand byte
- `AddInt`, `SubInt`, `MulInt`: integer fast path

---

## Build Configuration

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
```

Binary size: ~700KB

---

## Reproduce

```powershell
cargo build --release
Measure-Command { .\target\release\specter.exe --vm examples\bench_loop.sp }
```

---

## Comparison to Other Languages

| Language | Relative to SpecterScript |
|----------|--------------------------|
| Python 3.12 | 10x slower |
| Lua 5.4 | Similar speed |
| LuaJIT | 5x faster (has JIT) |
| JavaScript V8 | 3x faster (has JIT) |

SpecterScript is competitive with Lua for simple numeric workloads.
It does not claim to rival JIT-compiled languages.
