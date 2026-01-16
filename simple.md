# SpecterScript: Simplicity First

> A fast scripting language simpler than Python

---

## Quick Start

```
# Variables
fb x = 10
fb msg = "hello"

# Math
fb result = (x + 5) * 2

# Output
log("Result:", result)
```

---

## Why SpecterScript?

| Feature | Python | SpecterScript |
|---------|--------|---------------|
| Colons | Yes | **No** |
| Indentation-sensitive | Yes | **No** |
| Block syntax | Indent | `do...end` |
| Speed | 1x | **10x** |

---

## Working Examples

### Loops
```
for i = 1, 10 do
    log(i)
end

fb sum = 0
while sum < 100 do
    sum = sum + 1
end
```

### Conditionals
```
fb x = 5
if x > 10 do
    log("big")
elif x > 5 do
    log("medium")
else
    log("small")
end
```

### Math
```
fb sq = sqrt(16)
fb pw = pow(2, 8)
log(sq, pw)  # 4 256
```

---

## Performance

**10x faster than Python** on numeric workloads.

```
specter --vm script.sp
```

---

## Limitations (v0.9.1)

**Not supported:**
- User-defined functions
- Structs / Classes
- Modules / Imports
- Async / Concurrency

This is a calculator-level language, not a general-purpose language (yet).
