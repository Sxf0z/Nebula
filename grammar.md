# Nebula Language Reference
> **Version:** 3.0 (Simplified)
> **Extension:** `.na`

---

## 1. Variables (The "Simple" Style)
We removed declarations like `var` or `:=`. Just set the name and the value.

| Type | Keyword | Syntax Example |
| :--- | :--- | :--- |
| **Variable** | (None) | `score = 100` |
| **Permanent**| `perm` | `perm MAX_LIVES = 3` |

**Rules:**
1. If you write `x = 10` inside a function, it creates a new local variable `x`.
2. If `x` already exists in that function, it updates it.

---

## 2. Logic (The "Electric" Style)
Standard logic is replaced with state-based keywords.

| Concept | Keyword | Old World Equivalent |
| :--- | :--- | :--- |
| **True** | `on` | `true` |
| **False** | `off` | `false` |
| **Null** | `empty` | `nil` / `null` |

---

## 3. Keywords & Structure

| Keyword | Description |
| :--- | :--- |
| **Flow Control** | |
| `if` | Start check. |
| `elsif` | "Else If" - Check another condition. |
| `else` | Fallback. |
| `do` | Opens a block (loops, ifs, functions). |
| `end` | Closes a block. |
| **Loops** | |
| `while` | Loop while `on`. |
| `for` | Loop through list or range. |
| `in` | `for x in items`. |
| `break` | Stop loop. |
| `continue` | Skip iteration. |
| **Functions** | |
| `function` | Define a function. |
| `give` | Return a value (replaces `return`). |
| `import` | Load another file. |

---

## 4. Examples

### The "Hello World"
```nebula
function main() do
    log("System starting...")
    perm VERSION = "1.0"
    
    # Simple Variable
    status = on
    
    if status == on do
        log("Welcome to Nebula", VERSION)
    end
end