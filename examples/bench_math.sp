# Benchmark: Math operations
# Many arithmetic operations

fb result = 0
for i = 1, 10000 do
    fb x = i * 2
    fb y = x + 10
    fb z = y / 2
    result = result + z
end
log("Math result:", result)
