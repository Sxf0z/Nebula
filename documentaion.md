# Nebula Documentation

> **Nebula** is a compiled, high-performance scripting language. It combines Python's readability with the speed of compiled languages.

---

## Quick Start

```nebula
fn main() do
    log("Hello, Nebula!")
    
    for i = 1, 5 do
        log("Count:", i)
    end
end
```

Run: `nebula --vm script.na`

---

# Getting Started

## Installation

```bash
git clone https://github.com/Sxf0z/Nebula.git
cd nebula
cargo build --release
./target/release/nebula --help
```

## CLI Usage

| Command | Description |
|---------|-------------|
| `nebula script.na` | Run with interpreter |
| `nebula --vm script.na` | Run with bytecode VM (faster) |
| `nebula` | Interactive REPL |

---

# Language Reference

## Variables

```nebula
# Constants (immutable)
perm MAX_SIZE = 1000
perm PI = 3.14159

# Variables (mutable)
count = 0
count = count + 1
```

## Data Types

| Name | Keyword | Example |
|------|---------|---------|
| Integer | `int` | `42` |
| Float | `fl` | `3.14` |
| Boolean | `bool` | `on`, `off` |
| String | `wrd` | `"hello"` |
| List | `lst` | `[1, 2, 3]` |
| Map | `map` | `{"key": "value"}` |
| Null | `nil` | `empty` |

### Booleans: on / off

```nebula
perm active = on
perm paused = off

if active do
    log("Running")
end
```

## Operators

**Arithmetic**: `+`, `-`, `*`, `/`, `%`, `^`
**Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`
**Logical**: `and`, `or`, `!`
**Assignment**: `=`, `+=`, `-=`, `*=`, `/=`

---

# Collections

## Lists

```nebula
perm items = [1, 2, 3]
log(items[0])     # 1
items[1] = 99
log(len(items))   # 3
```

## Maps

```nebula
perm user = {"name": "Alex", "age": 25}
log(user["name"])  # Alex
user["age"] = 26
```

## Iteration

```nebula
each item in items do
    log(item)
end

for i = 0, 10 do
    log(i)
end
```

---

# Control Flow

## Conditionals

```nebula
if score >= 90 do
    log("A")
elsif score >= 80 do
    log("B")
else
    log("C")
end
```

## Loops

```nebula
# While
i = 0
while i < 5 do
    log(i)
    i = i + 1
end

# For (with step)
for i = 0, 10, 2 do
    log(i)
end

# Each
each x in [1, 2, 3] do
    log(x)
end
```

## Loop Control

```nebula
# Break
while on do
    if done do break end
end

# Continue
for i = 0, 10 do
    if i % 2 == 0 do continue end
    log(i)
end
```

## Pattern Matching

```nebula
match status do
    200 => log("OK"),
    404 => log("Not Found"),
    _ => log("Unknown"),
end
```

---

# Functions

## Definition

```nebula
fn add(a, b) do
    give a + b
end

# Short syntax
fn square(x) = x * x
```

## Closures

```nebula
perm factor = 2
fn scale(x) do
    give x * factor
end
```

## Recursion

```nebula
fn fib(n) do
    if n <= 1 do give n end
    give fib(n - 1) + fib(n - 2)
end
```

---

# Built-in Functions

| Function | Description |
|----------|-------------|
| `log(...)` | Print to stdout |
| `get()` | Read from stdin |
| `len(x)` | Collection length |
| `sqrt(x)` | Square root |
| `abs(x)` | Absolute value |
| `typeof(x)` | Type name |

---

# Advanced

## Structs

```nebula
struct User {
    name: wrd,
    age: int,
}

perm u = User { name: "Alex", age: 25 }
log(u.name)
```

## Error Handling

```nebula
try do
    risky_operation()
catch err do
    log("Error:", err)
end
```

## Async

```nebula
async fn fetch(url) do
    give await http_get(url)
end

spawn background_task()
```

---

# Engine Internals

## NanBoxing

All values are 64-bit. Floats stored directly; integers/booleans/pointers encoded in NaN space.

## String Interning

Strings are immutable and deduplicated. Comparison is O(1).

## Peephole Optimization

Bytecode patterns optimized at compile time (constant folding, specialized instructions).

## Global Indexing

Variables use direct array indices, no hash lookups at runtime.

---

## License

MIT License — **Built with 💜 and Rust**