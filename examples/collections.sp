# Collections example in SpecterScript

# Lists
fb numbers = lst(1, 2, 3, 4, 5)
log("Numbers:", numbers)

fb first = numbers[0]
log("First:", first)

# Iterate over list
log("Doubled:")
each n in numbers do
    log(n * 2)
end

# Maps
fb person = map("name": "Alice", "age": 30)
log("Person:", person)
log("Name:", person.name)
log("Age:", person.age)

# Ranges
log("Range 1 to 5:")
each i in 1..5 do
    log(i)
end

# Higher-order functions
fn double(x) = x * 2
log("double(5) =", double(5))
