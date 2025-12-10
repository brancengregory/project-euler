triangular_number <- function(n) { n * (n + 1) / 2 }
pentagonal_number <- function(n) { n * (3 * n - 1) / 2 }
hexagonal_number <- function(n) { n * (2 * n - 1) }

print(triangular_number(285))
print(pentagonal_number(165))
print(hexagonal_number(143))

special_seq <- function(f, max_value) {
	spec_seq <- f(1)

	while(max(spec_seq) < max_value) {
		spec_seq <- append(spec_seq, f(length(spec_seq) + 1))
	}

	return(spec_seq)
}

next_match <- function(x) {
	while (TRUE) {
		x <- x + 1
		print(x)
		h_n <- hexagonal_number(x)
		if (h_n %in% special_seq(pentagonal_number, h_n) & h_n %in% special_seq(triangular_number, h_n)) {
			return(h_n)
		}
	}
}
