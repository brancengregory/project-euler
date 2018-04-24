library(tidyverse)

data <- read.delim("~/Documents/project-euler/Problem 18/data", header=FALSE, stringsAsFactors=FALSE)
View(data)

likitysplit <- function (x) {
  stringr::str_split(x, " ")
}

string <- apply(data, 1, likitysplit)

mat <- matrix(nrow = nrow(data), ncol = nrow(data))

for (i in 1:nrow(mat)) {
  num <- str_split(string[[i]][[1]], " ") %>% lapply(as.integer)
  for (j in 1:length(num)) {
    mat[i, j] <- num[[j]]
  }
}
rm("i", "j", "num", "string")

optimalPath <- function (mat) {
  for (i in (nrow(mat)-1):1) {
    for (j in 1:i) {
      if ( mat[i+1, j] >= mat[i+1, j+1] ) {
        mat[i,j] <- mat[i,j] + mat[i+1, j]  
      } else {
        mat[i,j] <- mat[i,j] + mat[i+1, j+1]
      }
    }
  }
  return(mat[1,1])
}
