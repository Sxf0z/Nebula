# Python benchmark: bench_math.py
# Equivalent to SpecterScript bench_math.sp

total = 0
for i in range(1, 10001):
    a = i * 2
    b = a + i
    c = b - 1
    d = c * 2
    total = total + d

print(f"Math result: {total}")
