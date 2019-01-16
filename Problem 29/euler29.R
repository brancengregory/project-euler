a <- 2:100
b <- 2:100

nums <- c()

n <- 1
for (i in 1:length(a)) {
  for (j in 1:length(b)) {
    nums[n] <- a[i]^b[j]
    n <- n + 1
  }
}

ans <- length(unique(nums))
