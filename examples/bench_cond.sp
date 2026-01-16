# Benchmark: Conditional logic
# FizzBuzz-style conditionals

fb fizz = 0
fb buzz = 0
fb fizzbuzz = 0
for i = 1, 10000 do
    if i % 15 == 0 do
        fizzbuzz = fizzbuzz + 1
    elif i % 3 == 0 do
        fizz = fizz + 1
    elif i % 5 == 0 do
        buzz = buzz + 1
    end
end
log("FizzBuzz counts:", fizzbuzz, fizz, buzz)
