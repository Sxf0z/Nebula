# Fibonacci sequence in SpecterScript

fn fib(n: i64) -> i64:
    if n <= 1:
        return n
    return fib(n - 1) + fib(n - 2)

fn main():
    print("Fibonacci sequence:")
    for i in range(0, 15):
        result = fib(i)
        print(result)
