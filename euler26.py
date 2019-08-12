def primeSieve(n):
    check = [True] * n
    check[0] = check[1] = False
    primes = []
    for (i, x) in enumerate(check):
        if x:
            primes.append(i)
            for j in range(i*i, n, i):
                check[j] = False
    return primes

def getLongestPeriod(n):
    primes = primeSieve(n)
    for i in primes[::-1]:
        p = 1
        while 10**p % i != 1:
            p += 1
        if i-1 == p:
            return(i)

n = 1000
print(getLongestPeriod(1000))