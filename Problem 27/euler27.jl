function isPrime(x::Int64)
	if x <= 3
		return x > 1
	elseif x % 2 == 0 || x % 3 == 0
		return false
	end

	i = 5

	while i^2 <= x
		if x % i == 0 || x % (i+2) == 0
			return false
		end
		i += 6
	end

	return true
end

println(isPrime(1))