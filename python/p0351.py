
def Solution(n):
    if n % 2 == 0:
        numEvens = (n/2)-1
        numOdds = (n/2)-1 
    else:
        numEvens = (n//2) - 1
        numOdds = n - (n//2) - 1
    ans = (numOdds*6) + (numEvens*12) + 6
    return ans

print(Solution(10))