options(scipen = 999)

last_digits <- function(x, n) {
  as.character(x) |>
    strsplit("") |>
    unlist() |>
    tail(n) |>
    paste(collapse = "") |>
    as.numeric()
}

last_10_digits <- function(x) {
  last_digits(x, 10)
}

last_10_digits(65475148532515)
