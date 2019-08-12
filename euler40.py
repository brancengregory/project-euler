num = []
i = 1
while len(num)<1000000:
    temp = list(str(i))
    for j in temp:
        num.append(j)
    i += 1

ind = range(1, 7)
ind = [(10**i)-1 for i in ind]

ans = 1
for i in ind:
    ans = ans * int(num[i])

print(ans)