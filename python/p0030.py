powers = []
for i in range(10):
    powers.append(i**5)

def sumPowers(x):
    y = [powers[int(i)] for i in str(x)]
    return sum(y)

ans = []
for i in range(10, 1000000):
    if sumPowers(i) == i:
        ans.append(i)
    else:
        next

print(sum(ans))