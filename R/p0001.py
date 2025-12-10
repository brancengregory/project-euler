num <- seq(1, 999)
multiples.3 <- num[num %% 3 == 0]
multiples.5 <- num[num %% 5 == 0]
num.request <- union(multiples.3, multiples.5)
result <- sum(num.request)
print(result)