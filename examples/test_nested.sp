# Minimal nested loop test
fb sum = 0
for i = 1, 3 do
    for j = 1, 3 do
        sum = sum + 1
    end
end
log("sum:", sum)
