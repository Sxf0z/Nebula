# Enums and pattern matching in SpecterScript

enum Option<T>:
    Some(T)
    None

fn find_index(target: i64) -> Option<i64>:
    arr = [10, 20, 30, 40, 50]
    for i in range(0, 5):
        if arr[i] == target:
            return Option::Some(i)
    return Option::None

fn main():
    result = find_index(30)
    
    match result:
        Option::Some(idx) => print("Found at index: " + idx.to_string())
        Option::None => print("Not found")
