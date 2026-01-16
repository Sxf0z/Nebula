# SpecterScript Language Reference

> v1.0.0 - Stable Release

---

## Types

| Type | Example |
|------|---------|
| Number | `3.14`, `42` |
| String | `"hello"` |
| Boolean | `true`/`false` |
| Nil | `nil` |
| List | `[1, 2, 3]` |

---

## Variables

```
fb x = 10
fb name = "Alice"
cn PI = 3.14
```

---

## Functions

```
# Expression function
fn double(x) = x * 2

# Block function
fn greet(name) do
    log("Hello,", name)
end

# Calling functions
fb result = double(5)
greet("World")
```

---

## Operators

| Math | Comparison | Logic |
|------|------------|-------|
| `+` `-` `*` `/` `%` | `==` `!=` `<` `>` `<=` `>=` | `!` |

---

## Control Flow

```
# If statement
if condition do
    ...
elif other do
    ...
else
    ...
end

# While loop
while condition do
    ...
end

# For loop
for i = 1, 10 do
    ...
end
```

---

## Built-in Functions

| Function | Description |
|----------|-------------|
| `log(...)` | Print values |
| `typeof(x)` | Get type name |
| `len(x)` | Length |
| `sqrt(n)` | Square root |
| `abs(n)` | Absolute value |
| `floor(n)` | Round down |
| `ceil(n)` | Round up |
| `round(n)` | Round nearest |
| `pow(a, b)` | Power |
| `sin(n)` | Sine |
| `cos(n)` | Cosine |

---

## Limitations (v1.0)

**NOT implemented:**
- Closures (functions cannot capture outer variables)
- Structs / Enums
- Modules / Imports
- Async / Concurrency
- Try / Catch
