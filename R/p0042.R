library(readr)
library(purrr)
library(stringr)

words <- readr::read_lines("https://projecteuler.net/resources/documents/0042_words.txt") |>
	strsplit(",")

words <- sapply(words, \(x) gsub("\"", "", x))


triangle_number <- function(x) {
	0.5 * x * (x + 1)
}

triangle_seq <- function(x) {
	triangle_number(1:x)
}

letter_index <- function(x) {
	res <- numeric(length = length(x))

	for (i in seq_along(x)) {
		res[i] <- which(letters == x[i])
	}

	return(res)
}

data <- purrr::map_vec(
	tolower(words),
	~ stringr::str_split(.x, "") |>
		unlist() |>
		letter_index() |>
		sum()
)

lookup <- triangle_seq(max(data) + 1)

ans <- sum(data %in% lookup)
