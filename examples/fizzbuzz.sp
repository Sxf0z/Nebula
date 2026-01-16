# FizzBuzz in SpecterScript

for i = 1, 20 do
    if i % 15 == 0 do
        log("FizzBuzz")
    elif i % 3 == 0 do
        log("Fizz")
    elif i % 5 == 0 do
        log("Buzz")
    else
        log(i)
    end
end
