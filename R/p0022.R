library(tidyverse)

names <- read_csv("data.txt", col_names = FALSE, na = "")
names <- as.tibble(names)

names <- sort(names$X1)

values <- LETTERS
scores <- rep(0, length(names))

for (i in 1:length(names)) {
  sum <- 0
  for (j in 1:str_length(names[i])) {
    sum <- sum + which(values == strsplit(names[i], split = "")[[1]][j])
  }
  scores[i] <- sum * i
}
rm(i, j, sum)

cat('The answer is:', sum(scores))

