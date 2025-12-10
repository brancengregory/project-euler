library(progress)

isDivisible <- function(x, n) {
    if (x %% n == 0) {
        return(TRUE)
    } else {
        return(FALSE)
    }
}

streak <- function(x) {
    done <- FALSE
    i <- 1
    while(done == FALSE) {
        if(isDivisible(x, i)) {
            i <- i+1
        } else {
            i <- i-1
            return(i)
        }
    }
}

pFunc <- function(x, n) {
    count <- 0
    for(i in 2:n-1) {
        if(streak(i) == x) {
            count <- count+1
        }
    }
    return(count)
}

pb <- progress_bar$new(total = 31)
ans <- 0
for(i in 1:31){
    ans <- ans + pFunc(i, 4^i)
    pb$tick()
}