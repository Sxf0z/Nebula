# Fibonacci sequence in SpecterScript

fn fib(n) do
    fb a = 0
    fb b = 1
    for i = 0, n - 1 do
        fb temp = a + b
        a = b
        b = temp
    end
    -> a
end

log("Fibonacci sequence:")
for i = 0, 10 do
    log(i, "=>", fib(i))
end
