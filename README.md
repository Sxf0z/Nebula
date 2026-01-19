# Nebula 🌌



A compiled, high-performance scripting language.

---

## Features

| Feature | Description |
|---------|-------------|
| **State Logic** | `on` / `off` — No true/false |
| **Safety** | `empty` — No null pointer exceptions |
| **Speed** | 35x Faster Loops (Global Indexing) |
| **Optimization** | String Interning & Peephole Optimization |
| **Syntax** | Use `fn` for speed or `function` for clarity |

---

## Benchmarks (v1.0)

| Benchmark | Nebula | Python | Speedup |
|-----------|--------|--------|---------|
| **Fib(28)** | 0.05s | ~0.20s | **4x** |
| **100k Loops** | 0.03s | ~0.10s | **3x** |
| **Static Math** | 0.00s | 0.01s | **∞** (compile-time) |

---

## Quick Start

```nebula
fn main() do
    perm STATUS = on
    
    if STATUS == on do
        log("Nebula Initialized.")
    end
end
```

### Run a Script

```bash
# Build
cargo build --release

# Run with VM (fastest)
./target/release/nebula --vm script.na

# Run with interpreter
./target/release/nebula script.na
```

---

## Syntax Highlights

### Variables
```nebula
x = 42              # Implicit declaration
perm PI = 3.14159   # Constant (immutable)
```

### Functions
```nebula
fn double(x) do
    give x * 2
end

# Or use the verbose form:
function triple(x) do
    give x * 3
end
```

### Control Flow
```nebula
if x > 10 do
    log("Big")
elsif x > 5 do
    log("Medium")
else
    log("Small")
end
```

### Loops
```nebula
i = 0
while i < 10 do
    log(i)
    i = i + 1
end
```

---

## Installation

```bash
git clone https://github.com/Sxf0z/Nebula.git
cd nebula
cargo build --release
```

---

## License

MIT License

---

<div align="center">
  
**Built with 💜 and Rust**

</div>
