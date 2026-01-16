# Parity Test: Edge cases
# Tests various edge cases for interpreter/VM parity

# Test 1: Basic arithmetic
fb a = 10 + 20 * 2
log("arith:", a)

# Test 2: Comparison operators
fb gt = 5 > 3
fb lt = 5 < 3
fb eq = 5 == 5
log("cmp:", gt, lt, eq)

# Test 3: Nested loops
fb sum = 0
for i = 1, 3 do
    for j = 1, 3 do
        sum = sum + 1
    end
end
log("nested:", sum)

# Test 4: If-elif-else
fb x = 2
if x == 1 do
    log("one")
elif x == 2 do
    log("two")
else
    log("other")
end

# Test 5: While loop
fb count = 0
while count < 5 do
    count = count + 1
end
log("while:", count)

# Test 6: Negative numbers
fb neg = -10
fb abs_neg = abs(neg)
log("neg:", neg, abs_neg)

# Test 7: String operations
fb msg = "hello"
fb length = len(msg)
log("str:", msg, length)

# Test 8: Boolean logic
fb t = true
fb f = false
log("bool:", t, f, !t)

# Test 9: Division and modulo
fb div_result = 10 / 3
fb modulo = 10 % 3
log("divmod:", div_result, modulo)

# Test 10: Math builtins
fb sq = sqrt(16)
fb pw = pow(2, 8)
log("math:", sq, pw)
