# FizzBuzz in SpecterScript

fn main():
    for i in range(1, 101):
        result = if i % 15 == 0:
            "FizzBuzz"
        elif i % 3 == 0:
            "Fizz"
        elif i % 5 == 0:
            "Buzz"
        else:
            i.to_string()
        print(result)
