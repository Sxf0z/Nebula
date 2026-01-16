# Benchmark: Loop performance
# Counts from 1 to 100000

fb count = 0
for i = 1, 100000 do
    count = count + 1
end
log("Loop count:", count)
