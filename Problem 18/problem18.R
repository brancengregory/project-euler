library(tidyverse)

data <- read.delim("~/Documents/Project Euler/Problem 18/data", header=FALSE, stringsAsFactors=FALSE)
#View(data)

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

(nrow(mat)-1):1
