# Python benchmark: bench_loop.py
# Equivalent to SpecterScript bench_loop.sp

count = 0
for i in range(1, 100001):
    count = count + 1

print(f"Loop count: {count}")
